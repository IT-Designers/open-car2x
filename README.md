# Communication Library for Car2X projects

The code of this project was used for the communication in [LUKAS] and is now available as Open Source and licensed
under [GPLv3].

# LICENSE

Unless mentioned otherwise, all content of this repository is licensed under [GPLv3].

The copyright of some protocol definitions in `protocol/asn` are hold by ETSI or ISO:

- [cam-pdu-descriptions.asn](protocol/asn/cam-pdu-descriptions.asn): CAM EN 302 637-2, [Copyright 2019 ETSI], [src](https://forge.etsi.org/rep/ITS/asn1/cam_en302637_2/-/blob/7ae4195d48dd468754a50f1a3bb0c2ce976ae15a/CAM-PDU-Descriptions.asn)
- [cpm-pdu-descriptions.asn](protocol/asn/cpm-pdu-descriptions.asn): ETSI TR 103 562, [ETSI WorkItem 46541]
- [denm-pdu-descriptions.asn](protocol/asn/denm-pdu-descriptions.asn): ETSI EN 302 637-3, [Copyright 2019 ETSI], [src](https://forge.etsi.org/rep/ITS/asn1/denm_en302637_3/-/blob/29ec748fd9a0e44b91e1896867fa34453781e334/DENM-PDU-Descriptions.asn)
- [dsrc-simplified-v2.asn](protocol/asn/dsrc-simplified-v2.asn): ISO TS 19091 ([reduced](https://github.com/riebl/vanetza/blob/0051ac5b0382fcf14f1318e2abb1de9899caf51b/asn1/ISO_TS_19091_CPM.asn) due to technical limitations), [ISO`s Copyright]
- [its-container.asn](protocol/asn/its-container.asn): ETSI TS 102 894-2, [Copyright 2019 ETSI], [src](https://forge.etsi.org/rep/ITS/asn1/cdd_ts102894_2/blob/151b191121d05c3b808f5dec14387339730db14f/ITS-Container.asn)
- [mcm-pdu-descriptions.asn](protocol/asn/mcm-pdu-descriptions.asn): ETSI TR 103 563, [ETSI Stable Draft]
- [vam-pdu-descriptions.asn](protocol/asn/vam-pdu-descriptions.asn): ETSI TS 103 300-2, [Copyright 2020 ETSI], [src](https://forge.etsi.org/rep/ITS/asn1/vam-ts103300_3/blob/d88b54fdf7a0cba9b2fb071a1be753fdd113fa91/VAM-PDU-Descriptions.asn)
- [vam-temp-imports.asn](protocol/asn/vam-temp-imports.asn): ETSI TS 103 300-2 (prefixed with `Temp` due to technical limitations), [Copyright 2020 ETSI], [src](https://forge.etsi.org/rep/ITS/asn1/vam-ts103300_3/blob/d88b54fdf7a0cba9b2fb071a1be753fdd113fa91/VAM-Temp-Imports.asn)
- [vru-motorcyclist-special-container.asn](protocol/asn/vru-motorcyclist-special-container.asn): ETSI TS 103 300-3, [Copyright 2020 ETSI], [src](https://forge.etsi.org/rep/ITS/asn1/vam-ts103300_3/blob/d88b54fdf7a0cba9b2fb071a1be753fdd113fa91/motorcyclist-special-container.asn)

Note: Some ETSI protocols were altered to better suite the use cases of [LUKAS].

[LUKAS]: https://projekt-lukas.de
[GPLv3]: LICENSE

[Copyright 2019 ETSI]: LICENSE_ETSI_2019
[Copyright 2020 ETSI]: LICENSE_ETSI_2020
[ETSI Stable Draft]: LICENSE_ETSI_2019
[ETSI WorkItem 46541]: https://portal.etsi.org/webapp/WorkProgram/Report_WorkItem.asp?wki_id=46541
[ISO`s Copyright]: https://www.iso.org/terms-conditions-licence-agreement.html#Customer-Licence
