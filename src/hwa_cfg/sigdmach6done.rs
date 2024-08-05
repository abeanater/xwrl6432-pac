#[doc = "Register `SIGDMACH6DONE` reader"]
pub type R = crate::R<Sigdmach6doneSpec>;
#[doc = "Register `SIGDMACH6DONE` writer"]
pub type W = crate::W<Sigdmach6doneSpec>;
#[doc = "Field `SIGDMACH6DONE` reader - 31:0\\]
Signature for DMA channel 6 completion (tied to 0x0020 in HW)"]
pub type Sigdmach6doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH6DONE` writer - 31:0\\]
Signature for DMA channel 6 completion (tied to 0x0020 in HW)"]
pub type Sigdmach6doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 6 completion (tied to 0x0020 in HW)"]
    #[inline(always)]
    pub fn sigdmach6done(&self) -> Sigdmach6doneR {
        Sigdmach6doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 6 completion (tied to 0x0020 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach6done(&mut self) -> Sigdmach6doneW<Sigdmach6doneSpec> {
        Sigdmach6doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH6DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach6done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach6done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach6doneSpec;
impl crate::RegisterSpec for Sigdmach6doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach6done::R`](R) reader structure"]
impl crate::Readable for Sigdmach6doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach6done::W`](W) writer structure"]
impl crate::Writable for Sigdmach6doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH6DONE to value 0"]
impl crate::Resettable for Sigdmach6doneSpec {
    const RESET_VALUE: u32 = 0;
}
