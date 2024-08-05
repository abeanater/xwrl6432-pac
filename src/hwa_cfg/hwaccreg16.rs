#[doc = "Register `HWACCREG16` reader"]
pub type R = crate::R<Hwaccreg16Spec>;
#[doc = "Register `HWACCREG16` writer"]
pub type W = crate::W<Hwaccreg16Spec>;
#[doc = "Field `NLOOPS` reader - 11:0\\]
Number of loops: This register controls the number of times the State Machine will loop through the parameter-sets (from a programmed start index till a programmed end index) and run them. The maximum number of times the loop can be made is run is 4094. A value of zero programmed in this register means that the looping mechanism is disabled."]
pub type NloopsR = crate::FieldReader<u16>;
#[doc = "Field `NLOOPS` writer - 11:0\\]
Number of loops: This register controls the number of times the State Machine will loop through the parameter-sets (from a programmed start index till a programmed end index) and run them. The maximum number of times the loop can be made is run is 4094. A value of zero programmed in this register means that the looping mechanism is disabled."]
pub type NloopsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PARAMSTART` reader - 16:12\\]
These registers are used to control the start and stop index of the parameter-set through which the state machine loops through. The state machine starts at the parameter-set specified by PARAM_START and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_STOP, it loops back to the start index as specified by PARAM_START."]
pub type ParamstartR = crate::FieldReader;
#[doc = "Field `PARAMSTART` writer - 16:12\\]
These registers are used to control the start and stop index of the parameter-set through which the state machine loops through. The state machine starts at the parameter-set specified by PARAM_START and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_STOP, it loops back to the start index as specified by PARAM_START."]
pub type ParamstartW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PARAMSTOP` reader - 21:17\\]
These registers are used to control the start and stop index of the parameter-set through which the state machine loops through. The state machine starts at the parameter-set specified by PARAM_START and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_STOP, it loops back to the start index as specified by PARAM_START."]
pub type ParamstopR = crate::FieldReader;
#[doc = "Field `PARAMSTOP` writer - 21:17\\]
These registers are used to control the start and stop index of the parameter-set through which the state machine loops through. The state machine starts at the parameter-set specified by PARAM_START and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_STOP, it loops back to the start index as specified by PARAM_START."]
pub type ParamstopW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u16>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Number of loops: This register controls the number of times the State Machine will loop through the parameter-sets (from a programmed start index till a programmed end index) and run them. The maximum number of times the loop can be made is run is 4094. A value of zero programmed in this register means that the looping mechanism is disabled."]
    #[inline(always)]
    pub fn nloops(&self) -> NloopsR {
        NloopsR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:16 - 16:12\\]
These registers are used to control the start and stop index of the parameter-set through which the state machine loops through. The state machine starts at the parameter-set specified by PARAM_START and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_STOP, it loops back to the start index as specified by PARAM_START."]
    #[inline(always)]
    pub fn paramstart(&self) -> ParamstartR {
        ParamstartR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - 21:17\\]
These registers are used to control the start and stop index of the parameter-set through which the state machine loops through. The state machine starts at the parameter-set specified by PARAM_START and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_STOP, it loops back to the start index as specified by PARAM_START."]
    #[inline(always)]
    pub fn paramstop(&self) -> ParamstopR {
        ParamstopR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Number of loops: This register controls the number of times the State Machine will loop through the parameter-sets (from a programmed start index till a programmed end index) and run them. The maximum number of times the loop can be made is run is 4094. A value of zero programmed in this register means that the looping mechanism is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn nloops(&mut self) -> NloopsW<Hwaccreg16Spec> {
        NloopsW::new(self, 0)
    }
    #[doc = "Bits 12:16 - 16:12\\]
These registers are used to control the start and stop index of the parameter-set through which the state machine loops through. The state machine starts at the parameter-set specified by PARAM_START and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_STOP, it loops back to the start index as specified by PARAM_START."]
    #[inline(always)]
    #[must_use]
    pub fn paramstart(&mut self) -> ParamstartW<Hwaccreg16Spec> {
        ParamstartW::new(self, 12)
    }
    #[doc = "Bits 17:21 - 21:17\\]
These registers are used to control the start and stop index of the parameter-set through which the state machine loops through. The state machine starts at the parameter-set specified by PARAM_START and loads each parameter-set one after another and runs the accelerator as per that configuration. When the state machine reaches the parameter-set specified by PARAM_STOP, it loops back to the start index as specified by PARAM_START."]
    #[inline(always)]
    #[must_use]
    pub fn paramstop(&mut self) -> ParamstopW<Hwaccreg16Spec> {
        ParamstopW::new(self, 17)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Hwaccreg16Spec> {
        Nu1W::new(self, 22)
    }
}
#[doc = "HWACCREG16\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg16Spec;
impl crate::RegisterSpec for Hwaccreg16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg16::R`](R) reader structure"]
impl crate::Readable for Hwaccreg16Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg16::W`](W) writer structure"]
impl crate::Writable for Hwaccreg16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG16 to value 0"]
impl crate::Resettable for Hwaccreg16Spec {
    const RESET_VALUE: u32 = 0;
}
