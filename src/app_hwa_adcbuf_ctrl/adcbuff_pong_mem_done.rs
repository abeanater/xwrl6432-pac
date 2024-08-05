#[doc = "Register `ADCBUFF_PONG_MEM_DONE` reader"]
pub type R = crate::R<AdcbuffPongMemDoneSpec>;
#[doc = "Register `ADCBUFF_PONG_MEM_DONE` writer"]
pub type W = crate::W<AdcbuffPongMemDoneSpec>;
#[doc = "Field `mem_init_done` reader - 0:0\\]
This field will be high once initialization of ADCBUF PONG Memory is finished. Writing '1' would clear the bit."]
pub type MemInitDoneR = crate::BitReader;
#[doc = "Field `mem_init_done` writer - 0:0\\]
This field will be high once initialization of ADCBUF PONG Memory is finished. Writing '1' would clear the bit."]
pub type MemInitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once initialization of ADCBUF PONG Memory is finished. Writing '1' would clear the bit."]
    #[inline(always)]
    pub fn mem_init_done(&self) -> MemInitDoneR {
        MemInitDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once initialization of ADCBUF PONG Memory is finished. Writing '1' would clear the bit."]
    #[inline(always)]
    #[must_use]
    pub fn mem_init_done(&mut self) -> MemInitDoneW<AdcbuffPongMemDoneSpec> {
        MemInitDoneW::new(self, 0)
    }
}
#[doc = "ADCBUFF_PONG_MEM_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbuff_pong_mem_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbuff_pong_mem_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcbuffPongMemDoneSpec;
impl crate::RegisterSpec for AdcbuffPongMemDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcbuff_pong_mem_done::R`](R) reader structure"]
impl crate::Readable for AdcbuffPongMemDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`adcbuff_pong_mem_done::W`](W) writer structure"]
impl crate::Writable for AdcbuffPongMemDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCBUFF_PONG_MEM_DONE to value 0"]
impl crate::Resettable for AdcbuffPongMemDoneSpec {
    const RESET_VALUE: u32 = 0;
}
