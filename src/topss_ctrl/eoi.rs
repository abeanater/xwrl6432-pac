#[doc = "Register `eoi` reader"]
pub type R = crate::R<EoiSpec>;
#[doc = "Register `eoi` writer"]
pub type W = crate::W<EoiSpec>;
#[doc = "Field `EOI_VECTOR_VALUE_` reader - 7:0\\]
EOI vector value. Write this with interrupt distribution value in the chip."]
pub type EoiVectorValue_R = crate::FieldReader;
#[doc = "Field `EOI_VECTOR_VALUE_` writer - 7:0\\]
EOI vector value. Write this with interrupt distribution value in the chip."]
pub type EoiVectorValue_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
EOI vector value. Write this with interrupt distribution value in the chip."]
    #[inline(always)]
    pub fn eoi_vector_value_(&self) -> EoiVectorValue_R {
        EoiVectorValue_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
EOI vector value. Write this with interrupt distribution value in the chip."]
    #[inline(always)]
    #[must_use]
    pub fn eoi_vector_value_(&mut self) -> EoiVectorValue_W<EoiSpec> {
        EoiVectorValue_W::new(self, 0)
    }
}
#[doc = "EOI register\n\nYou can [`read`](crate::Reg::read) this register and get [`eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EoiSpec;
impl crate::RegisterSpec for EoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eoi::R`](R) reader structure"]
impl crate::Readable for EoiSpec {}
#[doc = "`write(|w| ..)` method takes [`eoi::W`](W) writer structure"]
impl crate::Writable for EoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets eoi to value 0"]
impl crate::Resettable for EoiSpec {
    const RESET_VALUE: u32 = 0;
}
