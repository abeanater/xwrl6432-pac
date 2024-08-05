#[doc = "Register `INTC_EOI` reader"]
pub type R = crate::R<IntcEoiSpec>;
#[doc = "Register `INTC_EOI` writer"]
pub type W = crate::W<IntcEoiSpec>;
#[doc = "Field `EOI_VECTOR` reader - 31:0\\]
Number associated with the ipgenericirq for intr output. There are 1 interrupt outputs Write 0x0 : Write to intr IP Generic Any other write value is ignored."]
pub type EoiVectorR = crate::FieldReader<u32>;
#[doc = "Field `EOI_VECTOR` writer - 31:0\\]
Number associated with the ipgenericirq for intr output. There are 1 interrupt outputs Write 0x0 : Write to intr IP Generic Any other write value is ignored."]
pub type EoiVectorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Number associated with the ipgenericirq for intr output. There are 1 interrupt outputs Write 0x0 : Write to intr IP Generic Any other write value is ignored."]
    #[inline(always)]
    pub fn eoi_vector(&self) -> EoiVectorR {
        EoiVectorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Number associated with the ipgenericirq for intr output. There are 1 interrupt outputs Write 0x0 : Write to intr IP Generic Any other write value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn eoi_vector(&mut self) -> EoiVectorW<IntcEoiSpec> {
        EoiVectorW::new(self, 0)
    }
}
#[doc = "EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcEoiSpec;
impl crate::RegisterSpec for IntcEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_eoi::R`](R) reader structure"]
impl crate::Readable for IntcEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`intc_eoi::W`](W) writer structure"]
impl crate::Writable for IntcEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTC_EOI to value 0"]
impl crate::Resettable for IntcEoiSpec {
    const RESET_VALUE: u32 = 0;
}
