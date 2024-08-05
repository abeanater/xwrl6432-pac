#[doc = "Register `SIGDMACH1DONE` reader"]
pub type R = crate::R<Sigdmach1doneSpec>;
#[doc = "Register `SIGDMACH1DONE` writer"]
pub type W = crate::W<Sigdmach1doneSpec>;
#[doc = "Field `SIGDMACH1DONE` reader - 31:0\\]
Signature for DMA channel 1 completion (tied to 0x0001 in HW). Linked DMA can copy from one of these SIG_DMACHx_DONE registers into DMA2ACC_TRIG register to set the appropriate register bit to signal the completion of DMA and trigger the accelerator"]
pub type Sigdmach1doneR = crate::FieldReader<u32>;
#[doc = "Field `SIGDMACH1DONE` writer - 31:0\\]
Signature for DMA channel 1 completion (tied to 0x0001 in HW). Linked DMA can copy from one of these SIG_DMACHx_DONE registers into DMA2ACC_TRIG register to set the appropriate register bit to signal the completion of DMA and trigger the accelerator"]
pub type Sigdmach1doneW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 1 completion (tied to 0x0001 in HW). Linked DMA can copy from one of these SIG_DMACHx_DONE registers into DMA2ACC_TRIG register to set the appropriate register bit to signal the completion of DMA and trigger the accelerator"]
    #[inline(always)]
    pub fn sigdmach1done(&self) -> Sigdmach1doneR {
        Sigdmach1doneR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Signature for DMA channel 1 completion (tied to 0x0001 in HW). Linked DMA can copy from one of these SIG_DMACHx_DONE registers into DMA2ACC_TRIG register to set the appropriate register bit to signal the completion of DMA and trigger the accelerator"]
    #[inline(always)]
    #[must_use]
    pub fn sigdmach1done(&mut self) -> Sigdmach1doneW<Sigdmach1doneSpec> {
        Sigdmach1doneW::new(self, 0)
    }
}
#[doc = "SIGDMACH1DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`sigdmach1done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigdmach1done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sigdmach1doneSpec;
impl crate::RegisterSpec for Sigdmach1doneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigdmach1done::R`](R) reader structure"]
impl crate::Readable for Sigdmach1doneSpec {}
#[doc = "`write(|w| ..)` method takes [`sigdmach1done::W`](W) writer structure"]
impl crate::Writable for Sigdmach1doneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGDMACH1DONE to value 0"]
impl crate::Resettable for Sigdmach1doneSpec {
    const RESET_VALUE: u32 = 0;
}
