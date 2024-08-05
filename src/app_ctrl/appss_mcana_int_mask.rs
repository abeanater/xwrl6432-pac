#[doc = "Register `APPSS_MCANA_INT_MASK` reader"]
pub type R = crate::R<AppssMcanaIntMaskSpec>;
#[doc = "Register `APPSS_MCANA_INT_MASK` writer"]
pub type W = crate::W<AppssMcanaIntMaskSpec>;
#[doc = "Field `mcan_int_mask` reader - 31:0\\]
Interrupt Mask for 32 MCANSS TX DMA interrupts. Writing 1'b1 to bit&lt;0-31> masks interrupt source &lt;0-31> respectively in MCANA"]
pub type McanIntMaskR = crate::FieldReader<u32>;
#[doc = "Field `mcan_int_mask` writer - 31:0\\]
Interrupt Mask for 32 MCANSS TX DMA interrupts. Writing 1'b1 to bit&lt;0-31> masks interrupt source &lt;0-31> respectively in MCANA"]
pub type McanIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt Mask for 32 MCANSS TX DMA interrupts. Writing 1'b1 to bit&lt;0-31> masks interrupt source &lt;0-31> respectively in MCANA"]
    #[inline(always)]
    pub fn mcan_int_mask(&self) -> McanIntMaskR {
        McanIntMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt Mask for 32 MCANSS TX DMA interrupts. Writing 1'b1 to bit&lt;0-31> masks interrupt source &lt;0-31> respectively in MCANA"]
    #[inline(always)]
    #[must_use]
    pub fn mcan_int_mask(&mut self) -> McanIntMaskW<AppssMcanaIntMaskSpec> {
        McanIntMaskW::new(self, 0)
    }
}
#[doc = "APPSS_MCANA_INT_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mcana_int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mcana_int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssMcanaIntMaskSpec;
impl crate::RegisterSpec for AppssMcanaIntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_mcana_int_mask::R`](R) reader structure"]
impl crate::Readable for AppssMcanaIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_mcana_int_mask::W`](W) writer structure"]
impl crate::Writable for AppssMcanaIntMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_MCANA_INT_MASK to value 0"]
impl crate::Resettable for AppssMcanaIntMaskSpec {
    const RESET_VALUE: u32 = 0;
}
