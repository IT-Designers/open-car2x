# `sample_client_c`

The `sample_client_c` directory contains an example for a c project using the connector library.
To make it work, you need to opy the relevant `.asn` files into the directory `proto/` and execute the following scripts:

```bash
cp -R ../protocol/asn sample_client_c/proto
./build_asn1c.sh
./build_and_run.sh
```
