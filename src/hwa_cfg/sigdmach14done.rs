#[doc = "Register `SIGDMACH14DONE` reader"]
pub type R = crate::R<Sigdmach14doneSpec>;
#[doc = "Register `SIGDMACH14DONE` writer"]
pub type W = crate::W<Sigdmach14doneSpec>;
#[doc = "Field `SIGDMACH14DONE` reader - 31:0\\]
Signature for DMA channel 14 completion (tied to 0x2000 in HW)"]
pub type Sigdmach14doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH14DONE` writer - 31:0\\]
Signature for DMA channel 14 completion (tied to 0x2000 in HW)"]
pub type Sigdmach14doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 14 completion (tied to 0x2000 in HW)"]
    #[inline(always)]
    pub fn sigdmach14done(&self) -> Sigdmach14doneR {
        Sigdmach14doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 14 completion (tied to 0x2000 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach14done(&mut self) -> Sigdmach14doneW<Sigdmach14doneSpec> {
        Sigdmach14doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH14DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach14done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach14done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach14doneSpec;
impl crate::RegisterSpec for Sigdmach14doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach14done::R`](R) reader structure"]
impl crate::Readable for Sigdmach14doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach14done::W`](W) writer structure"]
impl crate::Writable for Sigdmach14doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH14DONE to value 0"]
impl crate::Resettable for Sigdmach14doneSpec {
    const RESET_VALUE: u32 = 0;
}
