use asn1rs::converter::Converter;
use asn1rs::gen::rust::RustCodeGenerator;
use asn1rs::gen::sql::SqlDefGenerator;

fn main() {
    let mut converter = Converter::default();

    std::fs::read_dir("../protocol/asn")
        .into_iter()
        .flat_map(|read_dir| {
            read_dir
                .into_iter()
                .flat_map(|dir_entry| dir_entry.into_iter())
                .flat_map(|entry| {
                    entry
                        .path()
                        .as_os_str()
                        .to_os_string()
                        .into_string()
                        .into_iter()
                })
                .filter(|entry| entry.ends_with(".asn1") || entry.ends_with(".asn"))
        })
        .for_each(|path| {
            println!("cargo:rerun-if-changed={}", path);
            if let Err(e) = converter.load_file(&path) {
                panic!("Loading of .asn1 file failed {}: {:?}", path, e);
            }
        });

    if let Err(e) = converter.to_rust(
        "src/",
        #[allow(unused)]
        |generator: &mut RustCodeGenerator| {
            #[cfg(feature = "derive-serde")]
            generator.add_global_derive("Serialize"); // Adds serde_derive support: #[derive(Serialize)]
            #[cfg(feature = "derive-serde")]
            generator.add_global_derive("Deserialize"); // Adds serde_derive support: #[derive(Deserialize)]
        },
    ) {
        panic!("Conversion to rust failed: {:?}", e);
    }

    if let Err(e) = converter.to_protobuf("../protocol/proto/") {
        panic!("Conversion to proto failed: {:?}", e);
    }

    if let Err(e) = converter.to_sql_with(
        "../protocol/sql/",
        SqlDefGenerator::default()
            .optimize_tables_for_write_performance()
            .wrap_primary_key_on_overflow(),
    ) {
        panic!("Conversion to sql failed: {:?}", e);
    }
}
