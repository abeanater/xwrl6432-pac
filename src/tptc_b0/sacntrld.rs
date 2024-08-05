#[doc = "Register `SACNTRLD` reader"]
pub type R = crate::R<SacntrldSpec>;
#[doc = "Register `SACNTRLD` writer"]
pub type W = crate::W<SacntrldSpec>;
#[doc = "Field `ACNT_RELOAD_VALUE` reader - 15:0\\]
A-Cnt Reload value for Source Active Register set. Value copied from PCNT.ACNT: Represents the originally programmed value of ACNT.#br#The Reload value is used to reinitialize ACNT after each array is serviced \\[i.e. ACNT decrements to 0\\]. by the Src offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT bytes\\]"]
pub type AcntReloadValueR = crate::FieldReader<u16>;
#[doc = "Field `ACNT_RELOAD_VALUE` writer - 15:0\\]
A-Cnt Reload value for Source Active Register set. Value copied from PCNT.ACNT: Represents the originally programmed value of ACNT.#br#The Reload value is used to reinitialize ACNT after each array is serviced \\[i.e. ACNT decrements to 0\\]. by the Src offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT bytes\\]"]
pub type AcntReloadValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
A-Cnt Reload value for Source Active Register set. Value copied from PCNT.ACNT: Represents the originally programmed value of ACNT.#br#The Reload value is used to reinitialize ACNT after each array is serviced \\[i.e. ACNT decrements to 0\\]. by the Src offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT bytes\\]"]
    #[inline(always)]
    pub fn acnt_reload_value(&self) -> AcntReloadValueR {
        AcntReloadValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
A-Cnt Reload value for Source Active Register set. Value copied from PCNT.ACNT: Represents the originally programmed value of ACNT.#br#The Reload value is used to reinitialize ACNT after each array is serviced \\[i.e. ACNT decrements to 0\\]. by the Src offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT bytes\\]"]
    #[inline(always)]
    #[must_use]
    pub fn acnt_reload_value(&mut self) -> AcntReloadValueW<SacntrldSpec> {
        AcntReloadValueW::new(self, 0)
    }
}
#[doc = "Src Actv Set Cnt Reload\n\nYou can [`read`](crate::Reg::read) this register and get [`sacntrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sacntrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SacntrldSpec;
impl crate::RegisterSpec for SacntrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sacntrld::R`](R) reader structure"]
impl crate::Readable for SacntrldSpec {}
#[doc = "`write(|w| ..)` method takes [`sacntrld::W`](W) writer structure"]
impl crate::Writable for SacntrldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SACNTRLD to value 0"]
impl crate::Resettable for SacntrldSpec {
    const RESET_VALUE: u32 = 0;
}
