#[doc = "Register `SIGDMACH10DONE` reader"]
pub type R = crate::R<Sigdmach10doneSpec>;
#[doc = "Register `SIGDMACH10DONE` writer"]
pub type W = crate::W<Sigdmach10doneSpec>;
#[doc = "Field `SIGDMACH10DONE` reader - 31:0\\]
Signature for DMA channel 10 completion (tied to 0x0200 in HW)"]
pub type Sigdmach10doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH10DONE` writer - 31:0\\]
Signature for DMA channel 10 completion (tied to 0x0200 in HW)"]
pub type Sigdmach10doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 10 completion (tied to 0x0200 in HW)"]
    #[inline(always)]
    pub fn sigdmach10done(&self) -> Sigdmach10doneR {
        Sigdmach10doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 10 completion (tied to 0x0200 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach10done(&mut self) -> Sigdmach10doneW<Sigdmach10doneSpec> {
        Sigdmach10doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH10DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach10done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach10done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach10doneSpec;
impl crate::RegisterSpec for Sigdmach10doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach10done::R`](R) reader structure"]
impl crate::Readable for Sigdmach10doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach10done::W`](W) writer structure"]
impl crate::Writable for Sigdmach10doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH10DONE to value 0"]
impl crate::Resettable for Sigdmach10doneSpec {
    const RESET_VALUE: u32 = 0;
}
