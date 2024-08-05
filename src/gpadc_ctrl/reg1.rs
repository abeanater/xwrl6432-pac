#[doc = "Register `REG1` reader"]
pub type R = crate::R<Reg1Spec>;
#[doc = "Register `REG1` writer"]
pub type W = crate::W<Reg1Spec>;
#[doc = "Field `GPADC_TRIGGER` reader - 0:0\\]
Generates a single cycle pulse to trigger the IFM mode"]
pub type GpadcTriggerR = crate::BitReader;
#[doc = "Field `GPADC_TRIGGER` writer - 0:0\\]
Generates a single cycle pulse to trigger the IFM mode"]
pub type GpadcTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 7:1\\]
TI reserved"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:1\\]
TI reserved"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GPADC_INIT` reader - 8:8\\]
Resets the FSM and clears the data RAM"]
pub type GpadcInitR = crate::BitReader;
#[doc = "Field `GPADC_INIT` writer - 8:8\\]
Resets the FSM and clears the data RAM"]
pub type GpadcInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 15:9\\]
TI reserved"]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 15:9\\]
TI reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GPADC_FSM_BYPASS` reader - 16:16\\]
1:Bypass gpadc control .When bypassed start = gpadc_start_byp_val config_val = config_value_ifm param_val = param_val_ifm"]
pub type GpadcFsmBypassR = crate::BitReader;
#[doc = "Field `GPADC_FSM_BYPASS` writer - 16:16\\]
1:Bypass gpadc control .When bypassed start = gpadc_start_byp_val config_val = config_value_ifm param_val = param_val_ifm"]
pub type GpadcFsmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 23:17\\]
TI reserved"]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 23:17\\]
TI reserved"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `GPADC_START_BYP_VAL` reader - "]
pub type GpadcStartBypValR = crate::BitReader;
#[doc = "Field `GPADC_START_BYP_VAL` writer - "]
pub type GpadcStartBypValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU4` reader - 31:25\\]
TI reserved"]
pub type Nu4R = crate::FieldReader;
#[doc = "Field `NU4` writer - 31:25\\]
TI reserved"]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Generates a single cycle pulse to trigger the IFM mode"]
    #[inline(always)]
    pub fn gpadc_trigger(&self) -> GpadcTriggerR {
        GpadcTriggerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
TI reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Resets the FSM and clears the data RAM"]
    #[inline(always)]
    pub fn gpadc_init(&self) -> GpadcInitR {
        GpadcInitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
TI reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
1:Bypass gpadc control .When bypassed start = gpadc_start_byp_val config_val = config_value_ifm param_val = param_val_ifm"]
    #[inline(always)]
    pub fn gpadc_fsm_bypass(&self) -> GpadcFsmBypassR {
        GpadcFsmBypassR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
TI reserved"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn gpadc_start_byp_val(&self) -> GpadcStartBypValR {
        GpadcStartBypValR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
TI reserved"]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Generates a single cycle pulse to trigger the IFM mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_trigger(&mut self) -> GpadcTriggerW<Reg1Spec> {
        GpadcTriggerW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Reg1Spec> {
        Nu1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Resets the FSM and clears the data RAM"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_init(&mut self) -> GpadcInitW<Reg1Spec> {
        GpadcInitW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Reg1Spec> {
        Nu2W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
1:Bypass gpadc control .When bypassed start = gpadc_start_byp_val config_val = config_value_ifm param_val = param_val_ifm"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_fsm_bypass(&mut self) -> GpadcFsmBypassW<Reg1Spec> {
        GpadcFsmBypassW::new(self, 16)
    }
    #[doc = "Bits 17:23 - 23:17\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<Reg1Spec> {
        Nu3W::new(self, 17)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_start_byp_val(&mut self) -> GpadcStartBypValW<Reg1Spec> {
        GpadcStartBypValW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<Reg1Spec> {
        Nu4W::new(self, 25)
    }
}
#[doc = "gpadc start trigger for Inter frame mode\n\nYou can [`read`](crate::Reg::read) this register and get [`reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg1Spec;
impl crate::RegisterSpec for Reg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg1::R`](R) reader structure"]
impl crate::Readable for Reg1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg1::W`](W) writer structure"]
impl crate::Writable for Reg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG1 to value 0"]
impl crate::Resettable for Reg1Spec {
    const RESET_VALUE: u32 = 0;
}
