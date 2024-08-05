#[doc = "Register `SIGDMACH13DONE` reader"]
pub type R = crate::R<Sigdmach13doneSpec>;
#[doc = "Register `SIGDMACH13DONE` writer"]
pub type W = crate::W<Sigdmach13doneSpec>;
#[doc = "Field `SIGDMACH13DONE` reader - 31:0\\]
Signature for DMA channel 13 completion (tied to 0x1000 in HW)"]
pub type Sigdmach13doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH13DONE` writer - 31:0\\]
Signature for DMA channel 13 completion (tied to 0x1000 in HW)"]
pub type Sigdmach13doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 13 completion (tied to 0x1000 in HW)"]
    #[inline(always)]
    pub fn sigdmach13done(&self) -> Sigdmach13doneR {
        Sigdmach13doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 13 completion (tied to 0x1000 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach13done(&mut self) -> Sigdmach13doneW<Sigdmach13doneSpec> {
        Sigdmach13doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH13DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach13done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach13done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach13doneSpec;
impl crate::RegisterSpec for Sigdmach13doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach13done::R`](R) reader structure"]
impl crate::Readable for Sigdmach13doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach13done::W`](W) writer structure"]
impl crate::Writable for Sigdmach13doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH13DONE to value 0"]
impl crate::Resettable for Sigdmach13doneSpec {
    const RESET_VALUE: u32 = 0;
}