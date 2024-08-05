#[doc = "Register `APPSS_MCANA_INT_CLR` reader"]
pub type R = crate::R<AppssMcanaIntClrSpec>;
#[doc = "Register `APPSS_MCANA_INT_CLR` writer"]
pub type W = crate::W<AppssMcanaIntClrSpec>;
#[doc = "Field `mcan_int_clr` reader - 31:0\\]
Interrupt Clear for 32 MCANSS TX DMA interrupts. Writing 1'b1 to bit&lt;0-31> clears interrupt source &lt;0-31> respectively in MCANA"]
pub type McanIntClrR = crate::FieldReader<u32>;
#[doc = "Field `mcan_int_clr` writer - 31:0\\]
Interrupt Clear for 32 MCANSS TX DMA interrupts. Writing 1'b1 to bit&lt;0-31> clears interrupt source &lt;0-31> respectively in MCANA"]
pub type McanIntClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt Clear for 32 MCANSS TX DMA interrupts. Writing 1'b1 to bit&lt;0-31> clears interrupt source &lt;0-31> respectively in MCANA"]
    #[inline(always)]
    pub fn mcan_int_clr(&self) -> McanIntClrR {
        McanIntClrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt Clear for 32 MCANSS TX DMA interrupts. Writing 1'b1 to bit&lt;0-31> clears interrupt source &lt;0-31> respectively in MCANA"]
    #[inline(always)]
    #[must_use]
    pub fn mcan_int_clr(&mut self) -> McanIntClrW<AppssMcanaIntClrSpec> {
        McanIntClrW::new(self, 0)
    }
}
#[doc = "APPSS_MCANA_INT_CLR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mcana_int_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mcana_int_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssMcanaIntClrSpec;
impl crate::RegisterSpec for AppssMcanaIntClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_mcana_int_clr::R`](R) reader structure"]
impl crate::Readable for AppssMcanaIntClrSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_mcana_int_clr::W`](W) writer structure"]
impl crate::Writable for AppssMcanaIntClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_MCANA_INT_CLR to value 0"]
impl crate::Resettable for AppssMcanaIntClrSpec {
    const RESET_VALUE: u32 = 0;
}
