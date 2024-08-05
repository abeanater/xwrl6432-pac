#[doc = "Register `DFCNTRLD` reader"]
pub type R = crate::R<DfcntrldSpec>;
#[doc = "Register `DFCNTRLD` writer"]
pub type W = crate::W<DfcntrldSpec>;
#[doc = "Field `ACNT_RELOAD_VALUE_1` reader - 15:0\\]
A-Cnt Reload value for Destination FIFO Register set. Value copied from PCNT.ACNT: Represents the originally programmed value of ACNT.#br#The Reload value is used to reinitialize ACNT after each array is serviced \\[i.e. ACNT decrements to 0\\]. by the Src offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT bytes\\]"]
pub type AcntReloadValue1R = crate::FieldReader<u16>;
#[doc = "Field `ACNT_RELOAD_VALUE_1` writer - 15:0\\]
A-Cnt Reload value for Destination FIFO Register set. Value copied from PCNT.ACNT: Represents the originally programmed value of ACNT.#br#The Reload value is used to reinitialize ACNT after each array is serviced \\[i.e. ACNT decrements to 0\\]. by the Src offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT bytes\\]"]
pub type AcntReloadValue1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
A-Cnt Reload value for Destination FIFO Register set. Value copied from PCNT.ACNT: Represents the originally programmed value of ACNT.#br#The Reload value is used to reinitialize ACNT after each array is serviced \\[i.e. ACNT decrements to 0\\]. by the Src offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT bytes\\]"]
    #[inline(always)]
    pub fn acnt_reload_value_1(&self) -> AcntReloadValue1R {
        AcntReloadValue1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
A-Cnt Reload value for Destination FIFO Register set. Value copied from PCNT.ACNT: Represents the originally programmed value of ACNT.#br#The Reload value is used to reinitialize ACNT after each array is serviced \\[i.e. ACNT decrements to 0\\]. by the Src offset in bytes between the starting address of each source array \\[recall that there are BCNT arrays of ACNT bytes\\]"]
    #[inline(always)]
    #[must_use]
    pub fn acnt_reload_value_1(&mut self) -> AcntReloadValue1W<DfcntrldSpec> {
        AcntReloadValue1W::new(self, 0)
    }
}
#[doc = "Dst FIFO Set Cnt Reload\n\nYou can [`read`](crate::Reg::read) this register and get [`dfcntrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfcntrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfcntrldSpec;
impl crate::RegisterSpec for DfcntrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfcntrld::R`](R) reader structure"]
impl crate::Readable for DfcntrldSpec {}
#[doc = "`write(|w| ..)` method takes [`dfcntrld::W`](W) writer structure"]
impl crate::Writable for DfcntrldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFCNTRLD to value 0"]
impl crate::Resettable for DfcntrldSpec {
    const RESET_VALUE: u32 = 0;
}
