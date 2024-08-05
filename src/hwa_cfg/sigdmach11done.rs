#[doc = "Register `SIGDMACH11DONE` reader"]
pub type R = crate::R<Sigdmach11doneSpec>;
#[doc = "Register `SIGDMACH11DONE` writer"]
pub type W = crate::W<Sigdmach11doneSpec>;
#[doc = "Field `SIGDMACH11DONE` reader - 31:0\\]
Signature for DMA channel 11 completion (tied to 0x0040 in HW)"]
pub type Sigdmach11doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH11DONE` writer - 31:0\\]
Signature for DMA channel 11 completion (tied to 0x0040 in HW)"]
pub type Sigdmach11doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 11 completion (tied to 0x0040 in HW)"]
    #[inline(always)]
    pub fn sigdmach11done(&self) -> Sigdmach11doneR {
        Sigdmach11doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 11 completion (tied to 0x0040 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach11done(&mut self) -> Sigdmach11doneW<Sigdmach11doneSpec> {
        Sigdmach11doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH11DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach11done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach11done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach11doneSpec;
impl crate::RegisterSpec for Sigdmach11doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach11done::R`](R) reader structure"]
impl crate::Readable for Sigdmach11doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach11done::W`](W) writer structure"]
impl crate::Writable for Sigdmach11doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH11DONE to value 0"]
impl crate::Resettable for Sigdmach11doneSpec {
    const RESET_VALUE: u32 = 0;
}
