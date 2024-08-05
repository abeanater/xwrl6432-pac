#[doc = "Register `SIGDMACH15DONE` reader"]
pub type R = crate::R<Sigdmach15doneSpec>;
#[doc = "Register `SIGDMACH15DONE` writer"]
pub type W = crate::W<Sigdmach15doneSpec>;
#[doc = "Field `SIGDMACH15DONE` reader - 31:0\\]
Signature for DMA channel 15 completion (tied to 0x4000 in HW)"]
pub type Sigdmach15doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH15DONE` writer - 31:0\\]
Signature for DMA channel 15 completion (tied to 0x4000 in HW)"]
pub type Sigdmach15doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 15 completion (tied to 0x4000 in HW)"]
    #[inline(always)]
    pub fn sigdmach15done(&self) -> Sigdmach15doneR {
        Sigdmach15doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 15 completion (tied to 0x4000 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach15done(&mut self) -> Sigdmach15doneW<Sigdmach15doneSpec> {
        Sigdmach15doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH15DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach15done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach15done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach15doneSpec;
impl crate::RegisterSpec for Sigdmach15doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach15done::R`](R) reader structure"]
impl crate::Readable for Sigdmach15doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach15done::W`](W) writer structure"]
impl crate::Writable for Sigdmach15doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH15DONE to value 0"]
impl crate::Resettable for Sigdmach15doneSpec {
    const RESET_VALUE: u32 = 0;
}
