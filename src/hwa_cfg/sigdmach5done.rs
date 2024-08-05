#[doc = "Register `SIGDMACH5DONE` reader"]
pub type R = crate::R<Sigdmach5doneSpec>;
#[doc = "Register `SIGDMACH5DONE` writer"]
pub type W = crate::W<Sigdmach5doneSpec>;
#[doc = "Field `SIGDMACH5DONE` reader - 31:0\\]
Signature for DMA channel 5 completion (tied to 0x0010 in HW)"]
pub type Sigdmach5doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH5DONE` writer - 31:0\\]
Signature for DMA channel 5 completion (tied to 0x0010 in HW)"]
pub type Sigdmach5doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 5 completion (tied to 0x0010 in HW)"]
    #[inline(always)]
    pub fn sigdmach5done(&self) -> Sigdmach5doneR {
        Sigdmach5doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 5 completion (tied to 0x0010 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach5done(&mut self) -> Sigdmach5doneW<Sigdmach5doneSpec> {
        Sigdmach5doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH5DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach5done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach5done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach5doneSpec;
impl crate::RegisterSpec for Sigdmach5doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach5done::R`](R) reader structure"]
impl crate::Readable for Sigdmach5doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach5done::W`](W) writer structure"]
impl crate::Writable for Sigdmach5doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH5DONE to value 0"]
impl crate::Resettable for Sigdmach5doneSpec {
    const RESET_VALUE: u32 = 0;
}
