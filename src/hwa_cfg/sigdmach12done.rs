#[doc = "Register `SIGDMACH12DONE` reader"]
pub type R = crate::R<Sigdmach12doneSpec>;
#[doc = "Register `SIGDMACH12DONE` writer"]
pub type W = crate::W<Sigdmach12doneSpec>;
#[doc = "Field `SIGDMACH12DONE` reader - 31:0\\]
Signature for DMA channel 12 completion (tied to 0x0080 in HW)"]
pub type Sigdmach12doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH12DONE` writer - 31:0\\]
Signature for DMA channel 12 completion (tied to 0x0080 in HW)"]
pub type Sigdmach12doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 12 completion (tied to 0x0080 in HW)"]
    #[inline(always)]
    pub fn sigdmach12done(&self) -> Sigdmach12doneR {
        Sigdmach12doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 12 completion (tied to 0x0080 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach12done(&mut self) -> Sigdmach12doneW<Sigdmach12doneSpec> {
        Sigdmach12doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH12DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach12done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach12done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach12doneSpec;
impl crate::RegisterSpec for Sigdmach12doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach12done::R`](R) reader structure"]
impl crate::Readable for Sigdmach12doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach12done::W`](W) writer structure"]
impl crate::Writable for Sigdmach12doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH12DONE to value 0"]
impl crate::Resettable for Sigdmach12doneSpec {
    const RESET_VALUE: u32 = 0;
}
