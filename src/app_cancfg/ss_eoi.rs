#[doc = "Register `SS_EOI` reader"]
pub type R = crate::R<SsEoiSpec>;
#[doc = "Register `SS_EOI` writer"]
pub type W = crate::W<SsEoiSpec>;
#[doc = "Field `EOI` reader - 7:0\\]
Write with bit position of targeted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt. Field values: ext_ts_eoi(0): EOI value for External TS interrupt mcan_0_eoi(1): EOI value for mcan\\[0\\]
interrupt mcan_1_eoi(2): EOI value for mcan\\[1\\]
interrupt (EOI - End Of Interrupt)"]
pub type EoiR = crate::FieldReader;
#[doc = "Field `EOI` writer - 7:0\\]
Write with bit position of targeted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt. Field values: ext_ts_eoi(0): EOI value for External TS interrupt mcan_0_eoi(1): EOI value for mcan\\[0\\]
interrupt mcan_1_eoi(2): EOI value for mcan\\[1\\]
interrupt (EOI - End Of Interrupt)"]
pub type EoiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU7` reader - 31:8\\]
Reserved"]
pub type Nu7R = crate::FieldReader<u32>;
#[doc = "Field `NU7` writer - 31:8\\]
Reserved"]
pub type Nu7W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Write with bit position of targeted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt. Field values: ext_ts_eoi(0): EOI value for External TS interrupt mcan_0_eoi(1): EOI value for mcan\\[0\\]
interrupt mcan_1_eoi(2): EOI value for mcan\\[1\\]
interrupt (EOI - End Of Interrupt)"]
    #[inline(always)]
    pub fn eoi(&self) -> EoiR {
        EoiR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu7(&self) -> Nu7R {
        Nu7R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Write with bit position of targeted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt. Field values: ext_ts_eoi(0): EOI value for External TS interrupt mcan_0_eoi(1): EOI value for mcan\\[0\\]
interrupt mcan_1_eoi(2): EOI value for mcan\\[1\\]
interrupt (EOI - End Of Interrupt)"]
    #[inline(always)]
    #[must_use]
    pub fn eoi(&mut self) -> EoiW<SsEoiSpec> {
        EoiW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu7(&mut self) -> Nu7W<SsEoiSpec> {
        Nu7W::new(self, 8)
    }
}
#[doc = "SS_EOI\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsEoiSpec;
impl crate::RegisterSpec for SsEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_eoi::R`](R) reader structure"]
impl crate::Readable for SsEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_eoi::W`](W) writer structure"]
impl crate::Writable for SsEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_EOI to value 0"]
impl crate::Resettable for SsEoiSpec {
    const RESET_VALUE: u32 = 0;
}
