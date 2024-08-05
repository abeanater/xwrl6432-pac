#[doc = "Register `RST_FSM_TRIG` reader"]
pub type R = crate::R<RstFsmTrigSpec>;
#[doc = "Register `RST_FSM_TRIG` writer"]
pub type W = crate::W<RstFsmTrigSpec>;
#[doc = "Field `cpu` reader - 2:0\\]
FSM Reset Trigger"]
pub type CpuR = crate::FieldReader;
#[doc = "Field `cpu` writer - 2:0\\]
FSM Reset Trigger"]
pub type CpuW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
FSM Reset Trigger"]
    #[inline(always)]
    pub fn cpu(&self) -> CpuR {
        CpuR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
FSM Reset Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn cpu(&mut self) -> CpuW<RstFsmTrigSpec> {
        CpuW::new(self, 0)
    }
}
#[doc = "RST_FSM_TRIG\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_fsm_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_fsm_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstFsmTrigSpec;
impl crate::RegisterSpec for RstFsmTrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_fsm_trig::R`](R) reader structure"]
impl crate::Readable for RstFsmTrigSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_fsm_trig::W`](W) writer structure"]
impl crate::Writable for RstFsmTrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_FSM_TRIG to value 0"]
impl crate::Resettable for RstFsmTrigSpec {
    const RESET_VALUE: u32 = 0;
}
