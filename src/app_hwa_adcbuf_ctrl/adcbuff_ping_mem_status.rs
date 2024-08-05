#[doc = "Register `ADCBUFF_PING_MEM_STATUS` reader"]
pub type R = crate::R<AdcbuffPingMemStatusSpec>;
#[doc = "Register `ADCBUFF_PING_MEM_STATUS` writer"]
pub type W = crate::W<AdcbuffPingMemStatusSpec>;
#[doc = "Field `mem_init_status` reader - 0:0\\]
1'b0: No initialization is happening for ADCBUF PING Memory 1'b1: Initialization is in progress for ADCBUF PING Memory"]
pub type MemInitStatusR = crate::BitReader;
#[doc = "Field `mem_init_status` writer - 0:0\\]
1'b0: No initialization is happening for ADCBUF PING Memory 1'b1: Initialization is in progress for ADCBUF PING Memory"]
pub type MemInitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for ADCBUF PING Memory 1'b1: Initialization is in progress for ADCBUF PING Memory"]
    #[inline(always)]
    pub fn mem_init_status(&self) -> MemInitStatusR {
        MemInitStatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1'b0: No initialization is happening for ADCBUF PING Memory 1'b1: Initialization is in progress for ADCBUF PING Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_init_status(&mut self) -> MemInitStatusW<AdcbuffPingMemStatusSpec> {
        MemInitStatusW::new(self, 0)
    }
}
#[doc = "ADCBUFF_PING_MEM_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`adcbuff_ping_mem_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcbuff_ping_mem_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcbuffPingMemStatusSpec;
impl crate::RegisterSpec for AdcbuffPingMemStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcbuff_ping_mem_status::R`](R) reader structure"]
impl crate::Readable for AdcbuffPingMemStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`adcbuff_ping_mem_status::W`](W) writer structure"]
impl crate::Writable for AdcbuffPingMemStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCBUFF_PING_MEM_STATUS to value 0"]
impl crate::Resettable for AdcbuffPingMemStatusSpec {
    const RESET_VALUE: u32 = 0;
}
