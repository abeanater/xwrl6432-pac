# xwrl6432-pac
Peripheral Access Crate for usage on ti mmwave xwrl6432.  Generated using combination of tixml2svd and svd2pac.  Builds the groundwork for using rust on ti mmwave parts.

## Generation steps
```shell
 tixml2svd  -z -i "C:\ti\ccs1271\ccs\ccs_base\common\targetdb\devices\iwrl6432.xml" > xml/iwrl6432.svd                          
pip install lxml 
python sanitize_xml.py
rm src
svd2rust -i xml\iwrl6432_updated.svd --target=cortex-m   
form -i lib.rs -o src/
cargo fmt
```