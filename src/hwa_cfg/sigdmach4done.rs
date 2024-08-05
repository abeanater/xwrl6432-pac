#[doc = "Register `SIGDMACH4DONE` reader"]
pub type R = crate::R<Sigdmach4doneSpec>;
#[doc = "Register `SIGDMACH4DONE` writer"]
pub type W = crate::W<Sigdmach4doneSpec>;
#[doc = "Field `SIGDMACH4DONE` reader - 31:0\\]
Signature for DMA channel 4 completion (tied to 0x0008 in HW)"]
pub type Sigdmach4doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH4DONE` writer - 31:0\\]
Signature for DMA channel 4 completion (tied to 0x0008 in HW)"]
pub type Sigdmach4doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 4 completion (tied to 0x0008 in HW)"]
    #[inline(always)]
    pub fn sigdmach4done(&self) -> Sigdmach4doneR {
        Sigdmach4doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 4 completion (tied to 0x0008 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach4done(&mut self) -> Sigdmach4doneW<Sigdmach4doneSpec> {
        Sigdmach4doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH4DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach4done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach4done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach4doneSpec;
impl crate::RegisterSpec for Sigdmach4doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach4done::R`](R) reader structure"]
impl crate::Readable for Sigdmach4doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach4done::W`](W) writer structure"]
impl crate::Writable for Sigdmach4doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH4DONE to value 0"]
impl crate::Resettable for Sigdmach4doneSpec {
    const RESET_VALUE: u32 = 0;
}
