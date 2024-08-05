#[doc = "Register `SIGDMACH16DONE` reader"]
pub type R = crate::R<Sigdmach16doneSpec>;
#[doc = "Register `SIGDMACH16DONE` writer"]
pub type W = crate::W<Sigdmach16doneSpec>;
#[doc = "Field `SIGDMACH16DONE` reader - 31:0\\]
Signature for DMA channel 16 completion (tied to 0x8000 in HW)"]
pub type Sigdmach16doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH16DONE` writer - 31:0\\]
Signature for DMA channel 16 completion (tied to 0x8000 in HW)"]
pub type Sigdmach16doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 16 completion (tied to 0x8000 in HW)"]
    #[inline(always)]
    pub fn sigdmach16done(&self) -> Sigdmach16doneR {
        Sigdmach16doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 16 completion (tied to 0x8000 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach16done(&mut self) -> Sigdmach16doneW<Sigdmach16doneSpec> {
        Sigdmach16doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH16DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach16done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach16done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach16doneSpec;
impl crate::RegisterSpec for Sigdmach16doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach16done::R`](R) reader structure"]
impl crate::Readable for Sigdmach16doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach16done::W`](W) writer structure"]
impl crate::Writable for Sigdmach16doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH16DONE to value 0"]
impl crate::Resettable for Sigdmach16doneSpec {
    const RESET_VALUE: u32 = 0;
}
