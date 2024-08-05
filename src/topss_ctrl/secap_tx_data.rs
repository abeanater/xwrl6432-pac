#[doc = "Register `SECAP_TX_DATA` reader"]
pub type R = crate::R<SecapTxDataSpec>;
#[doc = "Register `SECAP_TX_DATA` writer"]
pub type W = crate::W<SecapTxDataSpec>;
#[doc = "Field `JTAGTXDATA` reader - 31:0\\]
This register is used to pass data to the system security logic. The data is transmit from the external JTAG interface and hence is the Rx path for the SECAP interface."]
pub type JtagtxdataR = crate::FieldReader<u32>;
#[doc = "Field `JTAGTXDATA` writer - 31:0\\]
This register is used to pass data to the system security logic. The data is transmit from the external JTAG interface and hence is the Rx path for the SECAP interface."]
pub type JtagtxdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register is used to pass data to the system security logic. The data is transmit from the external JTAG interface and hence is the Rx path for the SECAP interface."]
    #[inline(always)]
    pub fn jtagtxdata(&self) -> JtagtxdataR {
        JtagtxdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register is used to pass data to the system security logic. The data is transmit from the external JTAG interface and hence is the Rx path for the SECAP interface."]
    #[inline(always)]
    #[must_use]
    pub fn jtagtxdata(&mut self) -> JtagtxdataW<SecapTxDataSpec> {
        JtagtxdataW::new(self, 0)
    }
}
#[doc = "SECAP_TX_DATA\n\nYou can [`read`](crate::Reg::read) this register and get [`secap_tx_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secap_tx_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecapTxDataSpec;
impl crate::RegisterSpec for SecapTxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secap_tx_data::R`](R) reader structure"]
impl crate::Readable for SecapTxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`secap_tx_data::W`](W) writer structure"]
impl crate::Writable for SecapTxDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECAP_TX_DATA to value 0"]
impl crate::Resettable for SecapTxDataSpec {
    const RESET_VALUE: u32 = 0;
}
