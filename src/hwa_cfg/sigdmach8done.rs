#[doc = "Register `SIGDMACH8DONE` reader"]
pub type R = crate::R<Sigdmach8doneSpec>;
#[doc = "Register `SIGDMACH8DONE` writer"]
pub type W = crate::W<Sigdmach8doneSpec>;
#[doc = "Field `SIGDMACH8DONE` reader - 31:0\\]
Signature for DMA channel 8 completion (tied to 0x0080 in HW)"]
pub type Sigdmach8doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH8DONE` writer - 31:0\\]
Signature for DMA channel 8 completion (tied to 0x0080 in HW)"]
pub type Sigdmach8doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 8 completion (tied to 0x0080 in HW)"]
    #[inline(always)]
    pub fn sigdmach8done(&self) -> Sigdmach8doneR {
        Sigdmach8doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 8 completion (tied to 0x0080 in HW)"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach8done(&mut self) -> Sigdmach8doneW<Sigdmach8doneSpec> {
        Sigdmach8doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH8DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach8done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach8done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach8doneSpec;
impl crate::RegisterSpec for Sigdmach8doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach8done::R`](R) reader structure"]
impl crate::Readable for Sigdmach8doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach8done::W`](W) writer structure"]
impl crate::Writable for Sigdmach8doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH8DONE to value 0"]
impl crate::Resettable for Sigdmach8doneSpec {
    const RESET_VALUE: u32 = 0;
}
