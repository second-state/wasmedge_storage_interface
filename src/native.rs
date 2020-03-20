extern "C" {
    pub fn ssvm_storage_beginStoreTx();
    pub fn ssvm_storage_storeI32();
    pub fn ssvm_storage_loadI32();
    pub fn ssvm_storage_storeI64();
    pub fn ssvm_storage_loadI64();
    pub fn ssvm_storage_endStoreTx();
}