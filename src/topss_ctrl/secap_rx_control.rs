#[doc = "Register `SECAP_RX_CONTROL` reader"]
pub type R = crate::R<SecapRxControlSpec>;
#[doc = "Register `SECAP_RX_CONTROL` writer"]
pub type W = crate::W<SecapRxControlSpec>;
#[doc = "Field `JTAGRXCONTROL` reader - 30:0\\]
This register is provides the handshake for the JTAGRXDATA Register and can also be used to pass control information from the system security logic"]
pub type JtagrxcontrolR = crate::FieldReader<u32>;
#[doc = "Field `JTAGRXCONTROL` writer - 30:0\\]
This register is provides the handshake for the JTAGRXDATA Register and can also be used to pass control information from the system security logic"]
pub type JtagrxcontrolW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `RXDATA_AVAIL` reader - 31:31\\]
Tx Interrupt to indicate avaliablity of RXDATA . 1 - RXDATA available ; 0 - RXDATA not available"]
pub type RxdataAvailR = crate::BitReader;
#[doc = "Field `RXDATA_AVAIL` writer - 31:31\\]
Tx Interrupt to indicate avaliablity of RXDATA . 1 - RXDATA available ; 0 - RXDATA not available"]
pub type RxdataAvailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - 30:0\\]
This register is provides the handshake for the JTAGRXDATA Register and can also be used to pass control information from the system security logic"]
    #[inline(always)]
    pub fn jtagrxcontrol(&self) -> JtagrxcontrolR {
        JtagrxcontrolR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Tx Interrupt to indicate avaliablity of RXDATA . 1 - RXDATA available ; 0 - RXDATA not available"]
    #[inline(always)]
    pub fn rxdata_avail(&self) -> RxdataAvailR {
        RxdataAvailR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - 30:0\\]
This register is provides the handshake for the JTAGRXDATA Register and can also be used to pass control information from the system security logic"]
    #[inline(always)]
    #[must_use]
    pub fn jtagrxcontrol(&mut self) -> JtagrxcontrolW<SecapRxControlSpec> {
        JtagrxcontrolW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Tx Interrupt to indicate avaliablity of RXDATA . 1 - RXDATA available ; 0 - RXDATA not available"]
    #[inline(always)]
    #[must_use]
    pub fn rxdata_avail(&mut self) -> RxdataAvailW<SecapRxControlSpec> {
        RxdataAvailW::new(self, 31)
    }
}
#[doc = "SECAP_RX_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`secap_rx_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secap_rx_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecapRxControlSpec;
impl crate::RegisterSpec for SecapRxControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secap_rx_control::R`](R) reader structure"]
impl crate::Readable for SecapRxControlSpec {}
#[doc = "`write(|w| ..)` method takes [`secap_rx_control::W`](W) writer structure"]
impl crate::Writable for SecapRxControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECAP_RX_CONTROL to value 0"]
impl crate::Resettable for SecapRxControlSpec {
    const RESET_VALUE: u32 = 0;
}
