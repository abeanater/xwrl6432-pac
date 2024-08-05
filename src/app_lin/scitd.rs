#[doc = "Register `SCITD` reader"]
pub type R = crate::R<ScitdSpec>;
#[doc = "Register `SCITD` writer"]
pub type W = crate::W<ScitdSpec>;
#[doc = "Field `TD` reader - 7:0\\]
Transmit data This bit is effective in SCI-compatible mode only. Data to be transmitted is written to this register. The transfer of data from this register to the transmit shift register SCITXSHF sets the TXRDY flag (SCIFLR.23), which indicates that SCITD is ready to be loaded with another byte of data. Note: If TX INT ENA (SCISETINT.8) is set, this data transfer also causes an interrupt. Note: Data written to the SCIRD register that is fewer than eight bits long must be right justified, but it does not need to be padded with leading zeros."]
pub type TdR = crate::FieldReader;
#[doc = "Field `TD` writer - 7:0\\]
Transmit data This bit is effective in SCI-compatible mode only. Data to be transmitted is written to this register. The transfer of data from this register to the transmit shift register SCITXSHF sets the TXRDY flag (SCIFLR.23), which indicates that SCITD is ready to be loaded with another byte of data. Note: If TX INT ENA (SCISETINT.8) is set, this data transfer also causes an interrupt. Note: Data written to the SCIRD register that is fewer than eight bits long must be right justified, but it does not need to be padded with leading zeros."]
pub type TdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit data This bit is effective in SCI-compatible mode only. Data to be transmitted is written to this register. The transfer of data from this register to the transmit shift register SCITXSHF sets the TXRDY flag (SCIFLR.23), which indicates that SCITD is ready to be loaded with another byte of data. Note: If TX INT ENA (SCISETINT.8) is set, this data transfer also causes an interrupt. Note: Data written to the SCIRD register that is fewer than eight bits long must be right justified, but it does not need to be padded with leading zeros."]
    #[inline(always)]
    pub fn td(&self) -> TdR {
        TdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit data This bit is effective in SCI-compatible mode only. Data to be transmitted is written to this register. The transfer of data from this register to the transmit shift register SCITXSHF sets the TXRDY flag (SCIFLR.23), which indicates that SCITD is ready to be loaded with another byte of data. Note: If TX INT ENA (SCISETINT.8) is set, this data transfer also causes an interrupt. Note: Data written to the SCIRD register that is fewer than eight bits long must be right justified, but it does not need to be padded with leading zeros."]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TdW<ScitdSpec> {
        TdW::new(self, 0)
    }
}
#[doc = "The SCITD register is where data to be transmitted is written to by application software.\n\nYou can [`read`](crate::Reg::read) this register and get [`scitd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scitd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScitdSpec;
impl crate::RegisterSpec for ScitdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scitd::R`](R) reader structure"]
impl crate::Readable for ScitdSpec {}
#[doc = "`write(|w| ..)` method takes [`scitd::W`](W) writer structure"]
impl crate::Writable for ScitdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCITD to value 0"]
impl crate::Resettable for ScitdSpec {
    const RESET_VALUE: u32 = 0;
}
