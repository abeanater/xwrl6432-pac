#[doc = "Register `SECAP_RX_DATA` reader"]
pub type R = crate::R<SecapRxDataSpec>;
#[doc = "Register `SECAP_RX_DATA` writer"]
pub type W = crate::W<SecapRxDataSpec>;
#[doc = "Field `JTAGRXDATA` reader - 31:0\\]
This register is used to pass data from the system security logic. The data is transmit from the SECAP interface to external JTAG interface and hence is the Tx path for the SECAP interface."]
pub type JtagrxdataR = crate::FieldReader<u32>;
#[doc = "Field `JTAGRXDATA` writer - 31:0\\]
This register is used to pass data from the system security logic. The data is transmit from the SECAP interface to external JTAG interface and hence is the Tx path for the SECAP interface."]
pub type JtagrxdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register is used to pass data from the system security logic. The data is transmit from the SECAP interface to external JTAG interface and hence is the Tx path for the SECAP interface."]
    #[inline(always)]
    pub fn jtagrxdata(&self) -> JtagrxdataR {
        JtagrxdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register is used to pass data from the system security logic. The data is transmit from the SECAP interface to external JTAG interface and hence is the Tx path for the SECAP interface."]
    #[inline(always)]
    #[must_use]
    pub fn jtagrxdata(&mut self) -> JtagrxdataW<SecapRxDataSpec> {
        JtagrxdataW::new(self, 0)
    }
}
#[doc = "SECAP_RX_DATA\n\nYou can [`read`](crate::Reg::read) this register and get [`secap_rx_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secap_rx_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecapRxDataSpec;
impl crate::RegisterSpec for SecapRxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secap_rx_data::R`](R) reader structure"]
impl crate::Readable for SecapRxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`secap_rx_data::W`](W) writer structure"]
impl crate::Writable for SecapRxDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECAP_RX_DATA to value 0"]
impl crate::Resettable for SecapRxDataSpec {
    const RESET_VALUE: u32 = 0;
}
