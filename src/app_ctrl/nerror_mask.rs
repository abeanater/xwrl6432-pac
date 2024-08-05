#[doc = "Register `NERROR_MASK` reader"]
pub type R = crate::R<NerrorMaskSpec>;
#[doc = "Register `NERROR_MASK` writer"]
pub type W = crate::W<NerrorMaskSpec>;
#[doc = "Field `mask` reader - 0:0\\]
writing 1'b1 will mask the Nerror propagation to pad Writing 1'b0 will unmask the Nerror propagation to pad"]
pub type MaskR = crate::BitReader;
#[doc = "Field `mask` writer - 0:0\\]
writing 1'b1 will mask the Nerror propagation to pad Writing 1'b0 will unmask the Nerror propagation to pad"]
pub type MaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 will mask the Nerror propagation to pad Writing 1'b0 will unmask the Nerror propagation to pad"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 will mask the Nerror propagation to pad Writing 1'b0 will unmask the Nerror propagation to pad"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<NerrorMaskSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "NERROR_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`nerror_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nerror_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NerrorMaskSpec;
impl crate::RegisterSpec for NerrorMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nerror_mask::R`](R) reader structure"]
impl crate::Readable for NerrorMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`nerror_mask::W`](W) writer structure"]
impl crate::Writable for NerrorMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NERROR_MASK to value 0"]
impl crate::Resettable for NerrorMaskSpec {
    const RESET_VALUE: u32 = 0;
}
