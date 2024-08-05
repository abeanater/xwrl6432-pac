#[doc = "Register `ETCLR_ETFRC` reader"]
pub type R = crate::R<EtclrEtfrcSpec>;
#[doc = "Register `ETCLR_ETFRC` writer"]
pub type W = crate::W<EtclrEtfrcSpec>;
#[doc = "Field `ETCLR_INT` reader - 0:0\\]
ePWM Interrupt (EPWMx_INT) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[INT\\]
flag bit and enable further interrupts pulses to be generated"]
pub type EtclrIntR = crate::BitReader;
#[doc = "Field `ETCLR_INT` writer - 0:0\\]
ePWM Interrupt (EPWMx_INT) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[INT\\]
flag bit and enable further interrupts pulses to be generated"]
pub type EtclrIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 1:1\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - 1:1\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETCLR_SOCA` reader - 2:2\\]
ePWM ADC Start-of-Conversion A (EPWMxSOCA) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[SOCA\\]
flag bit"]
pub type EtclrSocaR = crate::BitReader;
#[doc = "Field `ETCLR_SOCA` writer - 2:2\\]
ePWM ADC Start-of-Conversion A (EPWMxSOCA) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[SOCA\\]
flag bit"]
pub type EtclrSocaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETCLR_SOCB` reader - 3:3\\]
ePWM ADC Start-of-Conversion B (EPWMxSOCB) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[SOCB\\]
flag bit"]
pub type EtclrSocbR = crate::BitReader;
#[doc = "Field `ETCLR_SOCB` writer - 3:3\\]
ePWM ADC Start-of-Conversion B (EPWMxSOCB) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[SOCB\\]
flag bit"]
pub type EtclrSocbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 15:4\\]
Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - 15:4\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ETFRC_INT` reader - 16:16\\]
INT Force Bit. The interrupt will only be generated if the event is enabled in the ETSEL register. The INT flag bit will be set regardless. 0 Writing 0 to this bit will be ignored. Always reads back a 0. 1 Generates an interrupt on EPWMxINT and set the INT flag bit. This bit is used for test purposes"]
pub type EtfrcIntR = crate::BitReader;
#[doc = "Field `ETFRC_INT` writer - 16:16\\]
INT Force Bit. The interrupt will only be generated if the event is enabled in the ETSEL register. The INT flag bit will be set regardless. 0 Writing 0 to this bit will be ignored. Always reads back a 0. 1 Generates an interrupt on EPWMxINT and set the INT flag bit. This bit is used for test purposes"]
pub type EtfrcIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 17:17\\]
Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - 17:17\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETFRC_SOCA` reader - 18:18\\]
SOCA Force Bit. The SOCA pulse will only be generated if the event is enabled in the ETSEL register. The ETFLG\\[SOCA\\]
flag bit will be set regardless. 0 Writing 0 to this bit will be ignored. Always reads back a 0. 1 Generates a pulse on EPWMxSOCA and set the SOCAFLG bit. This bit is used for test purposes"]
pub type EtfrcSocaR = crate::BitReader;
#[doc = "Field `ETFRC_SOCA` writer - 18:18\\]
SOCA Force Bit. The SOCA pulse will only be generated if the event is enabled in the ETSEL register. The ETFLG\\[SOCA\\]
flag bit will be set regardless. 0 Writing 0 to this bit will be ignored. Always reads back a 0. 1 Generates a pulse on EPWMxSOCA and set the SOCAFLG bit. This bit is used for test purposes"]
pub type EtfrcSocaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETFRC_SOCB` reader - 19:19\\]
SOCB Force Bit. The SOCB pulse will only be generated if the event is enabled in the ETSEL register. The ETFLG\\[SOCB\\]
flag bit will be set regardless. 0 Has no effect. Always reads back a 0. 1 Generates a pulse on EPWMxSOCB and sets the SOCBFLG bit. This bit is used for test purposes"]
pub type EtfrcSocbR = crate::BitReader;
#[doc = "Field `ETFRC_SOCB` writer - 19:19\\]
SOCB Force Bit. The SOCB pulse will only be generated if the event is enabled in the ETSEL register. The ETFLG\\[SOCB\\]
flag bit will be set regardless. 0 Has no effect. Always reads back a 0. 1 Generates a pulse on EPWMxSOCB and sets the SOCBFLG bit. This bit is used for test purposes"]
pub type EtfrcSocbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - 31:20\\]
Reserved"]
pub type Reserved4R = crate::FieldReader<u16>;
#[doc = "Field `Reserved4` writer - 31:20\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ePWM Interrupt (EPWMx_INT) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[INT\\]
flag bit and enable further interrupts pulses to be generated"]
    #[inline(always)]
    pub fn etclr_int(&self) -> EtclrIntR {
        EtclrIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
ePWM ADC Start-of-Conversion A (EPWMxSOCA) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[SOCA\\]
flag bit"]
    #[inline(always)]
    pub fn etclr_soca(&self) -> EtclrSocaR {
        EtclrSocaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
ePWM ADC Start-of-Conversion B (EPWMxSOCB) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[SOCB\\]
flag bit"]
    #[inline(always)]
    pub fn etclr_socb(&self) -> EtclrSocbR {
        EtclrSocbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
INT Force Bit. The interrupt will only be generated if the event is enabled in the ETSEL register. The INT flag bit will be set regardless. 0 Writing 0 to this bit will be ignored. Always reads back a 0. 1 Generates an interrupt on EPWMxINT and set the INT flag bit. This bit is used for test purposes"]
    #[inline(always)]
    pub fn etfrc_int(&self) -> EtfrcIntR {
        EtfrcIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
SOCA Force Bit. The SOCA pulse will only be generated if the event is enabled in the ETSEL register. The ETFLG\\[SOCA\\]
flag bit will be set regardless. 0 Writing 0 to this bit will be ignored. Always reads back a 0. 1 Generates a pulse on EPWMxSOCA and set the SOCAFLG bit. This bit is used for test purposes"]
    #[inline(always)]
    pub fn etfrc_soca(&self) -> EtfrcSocaR {
        EtfrcSocaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
SOCB Force Bit. The SOCB pulse will only be generated if the event is enabled in the ETSEL register. The ETFLG\\[SOCB\\]
flag bit will be set regardless. 0 Has no effect. Always reads back a 0. 1 Generates a pulse on EPWMxSOCB and sets the SOCBFLG bit. This bit is used for test purposes"]
    #[inline(always)]
    pub fn etfrc_socb(&self) -> EtfrcSocbR {
        EtfrcSocbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ePWM Interrupt (EPWMx_INT) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[INT\\]
flag bit and enable further interrupts pulses to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn etclr_int(&mut self) -> EtclrIntW<EtclrEtfrcSpec> {
        EtclrIntW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<EtclrEtfrcSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
ePWM ADC Start-of-Conversion A (EPWMxSOCA) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[SOCA\\]
flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn etclr_soca(&mut self) -> EtclrSocaW<EtclrEtfrcSpec> {
        EtclrSocaW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
ePWM ADC Start-of-Conversion B (EPWMxSOCB) Flag Clear Bit 0 Writing a 0 has no effect. Always reads back a 0 1 Clears the ETFLG\\[SOCB\\]
flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn etclr_socb(&mut self) -> EtclrSocbW<EtclrEtfrcSpec> {
        EtclrSocbW::new(self, 3)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<EtclrEtfrcSpec> {
        Reserved2W::new(self, 4)
    }
    #[doc = "Bit 16 - 16:16\\]
INT Force Bit. The interrupt will only be generated if the event is enabled in the ETSEL register. The INT flag bit will be set regardless. 0 Writing 0 to this bit will be ignored. Always reads back a 0. 1 Generates an interrupt on EPWMxINT and set the INT flag bit. This bit is used for test purposes"]
    #[inline(always)]
    #[must_use]
    pub fn etfrc_int(&mut self) -> EtfrcIntW<EtclrEtfrcSpec> {
        EtfrcIntW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<EtclrEtfrcSpec> {
        Reserved3W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
SOCA Force Bit. The SOCA pulse will only be generated if the event is enabled in the ETSEL register. The ETFLG\\[SOCA\\]
flag bit will be set regardless. 0 Writing 0 to this bit will be ignored. Always reads back a 0. 1 Generates a pulse on EPWMxSOCA and set the SOCAFLG bit. This bit is used for test purposes"]
    #[inline(always)]
    #[must_use]
    pub fn etfrc_soca(&mut self) -> EtfrcSocaW<EtclrEtfrcSpec> {
        EtfrcSocaW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
SOCB Force Bit. The SOCB pulse will only be generated if the event is enabled in the ETSEL register. The ETFLG\\[SOCB\\]
flag bit will be set regardless. 0 Has no effect. Always reads back a 0. 1 Generates a pulse on EPWMxSOCB and sets the SOCBFLG bit. This bit is used for test purposes"]
    #[inline(always)]
    #[must_use]
    pub fn etfrc_socb(&mut self) -> EtfrcSocbW<EtclrEtfrcSpec> {
        EtfrcSocbW::new(self, 19)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<EtclrEtfrcSpec> {
        Reserved4W::new(self, 20)
    }
}
#[doc = "Event-Trigger Clear Register/ Event-Trigger Force Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etclr_etfrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etclr_etfrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtclrEtfrcSpec;
impl crate::RegisterSpec for EtclrEtfrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etclr_etfrc::R`](R) reader structure"]
impl crate::Readable for EtclrEtfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`etclr_etfrc::W`](W) writer structure"]
impl crate::Writable for EtclrEtfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETCLR_ETFRC to value 0"]
impl crate::Resettable for EtclrEtfrcSpec {
    const RESET_VALUE: u32 = 0;
}
