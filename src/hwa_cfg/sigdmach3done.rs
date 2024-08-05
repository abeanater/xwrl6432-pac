#[doc = "Register `SIGDMACH3DONE` reader"]
pub type R = crate::R<Sigdmach3doneSpec>;
#[doc = "Register `SIGDMACH3DONE` writer"]
pub type W = crate::W<Sigdmach3doneSpec>;
#[doc = "Field `SIGDMACH3DONE` reader - 31:0\\]
Signature for DMA channel 3 completion (tied to 0x0004 in HW)"]
pub type Sigdmach3doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH3DONE` writer - 31:0\\]
Signature for DMA channel 3 completion (tied to 0x0004 in HW)"]
pub type Sigdmach3doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 3 completion (tied to 0x0004 in HW)"]
    #[inline(always)]
    pub fn sigdmach3done(&self) -> Sigdmach3doneR {
        Sigdmach3doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 3 completion (tied to 0x0004 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach3done(&mut self) -> Sigdmach3doneW<Sigdmach3doneSpec> {
        Sigdmach3doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH3DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach3done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach3done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach3doneSpec;
impl crate::RegisterSpec for Sigdmach3doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach3done::R`](R) reader structure"]
impl crate::Readable for Sigdmach3doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach3done::W`](W) writer structure"]
impl crate::Writable for Sigdmach3doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH3DONE to value 0"]
impl crate::Resettable for Sigdmach3doneSpec {
    const RESET_VALUE: u32 = 0;
}
