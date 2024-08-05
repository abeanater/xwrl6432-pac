#[doc = "Register `SIGDMACH7DONE` reader"]
pub type R = crate::R<Sigdmach7doneSpec>;
#[doc = "Register `SIGDMACH7DONE` writer"]
pub type W = crate::W<Sigdmach7doneSpec>;
#[doc = "Field `SIGDMACH7DONE` reader - 31:0\\]
Signature for DMA channel 7 completion (tied to 0x0040 in HW)"]
pub type Sigdmach7doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH7DONE` writer - 31:0\\]
Signature for DMA channel 7 completion (tied to 0x0040 in HW)"]
pub type Sigdmach7doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 7 completion (tied to 0x0040 in HW)"]
    #[inline(always)]
    pub fn sigdmach7done(&self) -> Sigdmach7doneR {
        Sigdmach7doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 7 completion (tied to 0x0040 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach7done(&mut self) -> Sigdmach7doneW<Sigdmach7doneSpec> {
        Sigdmach7doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH7DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach7done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach7done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach7doneSpec;
impl crate::RegisterSpec for Sigdmach7doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach7done::R`](R) reader structure"]
impl crate::Readable for Sigdmach7doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach7done::W`](W) writer structure"]
impl crate::Writable for Sigdmach7doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH7DONE to value 0"]
impl crate::Resettable for Sigdmach7doneSpec {
    const RESET_VALUE: u32 = 0;
}
