#[doc = "Register `SIGDMACH9DONE` reader"]
pub type R = crate::R<Sigdmach9doneSpec>;
#[doc = "Register `SIGDMACH9DONE` writer"]
pub type W = crate::W<Sigdmach9doneSpec>;
#[doc = "Field `SIGDMACH9DONE` reader - 31:0\\]
Signature for DMA channel 9 completion (tied to 0x0100 in HW)"]
pub type Sigdmach9doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH9DONE` writer - 31:0\\]
Signature for DMA channel 9 completion (tied to 0x0100 in HW)"]
pub type Sigdmach9doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 9 completion (tied to 0x0100 in HW)"]
    #[inline(always)]
    pub fn sigdmach9done(&self) -> Sigdmach9doneR {
        Sigdmach9doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 9 completion (tied to 0x0100 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach9done(&mut self) -> Sigdmach9doneW<Sigdmach9doneSpec> {
        Sigdmach9doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH9DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach9done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach9done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach9doneSpec;
impl crate::RegisterSpec for Sigdmach9doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach9done::R`](R) reader structure"]
impl crate::Readable for Sigdmach9doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach9done::W`](W) writer structure"]
impl crate::Writable for Sigdmach9doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH9DONE to value 0"]
impl crate::Resettable for Sigdmach9doneSpec {
    const RESET_VALUE: u32 = 0;
}
