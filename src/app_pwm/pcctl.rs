#[doc = "Register `PCCTL` reader"]
pub type R = crate::R<PcctlSpec>;
#[doc = "Register `PCCTL` writer"]
pub type W = crate::W<PcctlSpec>;
#[doc = "Field `CHPEN` reader - 0:0\\]
PWM-chopping Enable 0 Disable (bypass) PWM chopping function 1 Enable chopping function"]
pub type ChpenR = crate::BitReader;
#[doc = "Field `CHPEN` writer - 0:0\\]
PWM-chopping Enable 0 Disable (bypass) PWM chopping function 1 Enable chopping function"]
pub type ChpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSHTWTH` reader - 4:1\\]
One-Shot Pulse Width 0 1 x VCLK3 / 8 wide ( = 80 nS at 100 MHz VCLK3) 1h 2 x VCLK3 / 8 wide ( = 160 nS at 100 MHz VCLK3) 2h 3 x VCLK3 / 8 wide ( = 240 nS at 100 MHz VCLK3) 3h 4 x VCLK3 / 8 wide ( = 320 nS at 100 MHz VCLK3) 4h 5 x VCLK3 / 8 wide ( = 400 nS at 100 MHz VCLK3) 5h 6 x VCLK3 / 8 wide ( = 480 nS at 100 MHz VCLK3) 6h 7 x VCLK3 / 8 wide ( = 560 nS at 100 MHz VCLK3) 7h 8 x VCLK3 / 8 wide ( = 640 nS at 100 MHz VCLK3) 8h 9 x VCLK3 / 8 wide ( = 720 nS at 100 MHz VCLK3) 9h 10 x VCLK3 / 8 wide ( = 800 nS at 100 MHz VCLK3) Ah 11 x VCLK3 / 8 wide ( = 880 nS at 100 MHz VCLK3) Bh 12 x VCLK3 / 8 wide ( = 960 nS at 100 MHz VCLK3) Ch 13 x VCLK3 / 8 wide ( = 1040 nS at 100 MHz VCLK3) Dh 14 x VCLK3 / 8 wide ( = 1120 nS at 100 MHz VCLK3) Eh 15 x VCLK3 / 8 wide ( = 1200 nS at 100 MHz VCLK3) Fh 16 x VCLK3 / 8 wide ( = 1280 nS at 100 MHz VCLK3)"]
pub type OshtwthR = crate::FieldReader;
#[doc = "Field `OSHTWTH` writer - 4:1\\]
One-Shot Pulse Width 0 1 x VCLK3 / 8 wide ( = 80 nS at 100 MHz VCLK3) 1h 2 x VCLK3 / 8 wide ( = 160 nS at 100 MHz VCLK3) 2h 3 x VCLK3 / 8 wide ( = 240 nS at 100 MHz VCLK3) 3h 4 x VCLK3 / 8 wide ( = 320 nS at 100 MHz VCLK3) 4h 5 x VCLK3 / 8 wide ( = 400 nS at 100 MHz VCLK3) 5h 6 x VCLK3 / 8 wide ( = 480 nS at 100 MHz VCLK3) 6h 7 x VCLK3 / 8 wide ( = 560 nS at 100 MHz VCLK3) 7h 8 x VCLK3 / 8 wide ( = 640 nS at 100 MHz VCLK3) 8h 9 x VCLK3 / 8 wide ( = 720 nS at 100 MHz VCLK3) 9h 10 x VCLK3 / 8 wide ( = 800 nS at 100 MHz VCLK3) Ah 11 x VCLK3 / 8 wide ( = 880 nS at 100 MHz VCLK3) Bh 12 x VCLK3 / 8 wide ( = 960 nS at 100 MHz VCLK3) Ch 13 x VCLK3 / 8 wide ( = 1040 nS at 100 MHz VCLK3) Dh 14 x VCLK3 / 8 wide ( = 1120 nS at 100 MHz VCLK3) Eh 15 x VCLK3 / 8 wide ( = 1200 nS at 100 MHz VCLK3) Fh 16 x VCLK3 / 8 wide ( = 1280 nS at 100 MHz VCLK3)"]
pub type OshtwthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHPFREQ` reader - 7:5\\]
Chopping Clock Frequency 0 Divide by 1 (no prescale, = 12.5 MHz at 100 MHz VCLK3) 1h Divide by 2 (6.25 MHz at 100 MHz VCLK3) 2h Divide by 3 (4.16 MHz at 100 MHz VCLK3) 3h Divide by 4 (3.12 MHz at 100 MHz VCLK3) 4h Divide by 5 (2.50 MHz at 100 MHz VCLK3) 5h Divide by 6 (2.08 MHz at 100 MHz VCLK3) 6h Divide by 7 (1.78 MHz at 100 MHz VCLK3) 7h Divide by 8 (1.56 MHz at 100 MHz VCLK3)"]
pub type ChpfreqR = crate::FieldReader;
#[doc = "Field `CHPFREQ` writer - 7:5\\]
Chopping Clock Frequency 0 Divide by 1 (no prescale, = 12.5 MHz at 100 MHz VCLK3) 1h Divide by 2 (6.25 MHz at 100 MHz VCLK3) 2h Divide by 3 (4.16 MHz at 100 MHz VCLK3) 3h Divide by 4 (3.12 MHz at 100 MHz VCLK3) 4h Divide by 5 (2.50 MHz at 100 MHz VCLK3) 5h Divide by 6 (2.08 MHz at 100 MHz VCLK3) 6h Divide by 7 (1.78 MHz at 100 MHz VCLK3) 7h Divide by 8 (1.56 MHz at 100 MHz VCLK3)"]
pub type ChpfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CHPDUTY` reader - 10:8\\]
Chopping Clock Duty Cycle 0 Duty = 1/8 (12.5%) 1h Duty = 2/8 (25.0%) 2h Duty = 3/8 (37.5%) 3h Duty = 4/8 (50.0%) 4h Duty = 5/8 (62.5%) 5h Duty = 6/8 (75.0%) 6h Duty = 7/8 (87.5%) 7h Reserved"]
pub type ChpdutyR = crate::FieldReader;
#[doc = "Field `CHPDUTY` writer - 10:8\\]
Chopping Clock Duty Cycle 0 Duty = 1/8 (12.5%) 1h Duty = 2/8 (25.0%) 2h Duty = 3/8 (37.5%) 3h Duty = 4/8 (50.0%) 4h Duty = 5/8 (62.5%) 5h Duty = 6/8 (75.0%) 6h Duty = 7/8 (87.5%) 7h Reserved"]
pub type ChpdutyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PWM-chopping Enable 0 Disable (bypass) PWM chopping function 1 Enable chopping function"]
    #[inline(always)]
    pub fn chpen(&self) -> ChpenR {
        ChpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
One-Shot Pulse Width 0 1 x VCLK3 / 8 wide ( = 80 nS at 100 MHz VCLK3) 1h 2 x VCLK3 / 8 wide ( = 160 nS at 100 MHz VCLK3) 2h 3 x VCLK3 / 8 wide ( = 240 nS at 100 MHz VCLK3) 3h 4 x VCLK3 / 8 wide ( = 320 nS at 100 MHz VCLK3) 4h 5 x VCLK3 / 8 wide ( = 400 nS at 100 MHz VCLK3) 5h 6 x VCLK3 / 8 wide ( = 480 nS at 100 MHz VCLK3) 6h 7 x VCLK3 / 8 wide ( = 560 nS at 100 MHz VCLK3) 7h 8 x VCLK3 / 8 wide ( = 640 nS at 100 MHz VCLK3) 8h 9 x VCLK3 / 8 wide ( = 720 nS at 100 MHz VCLK3) 9h 10 x VCLK3 / 8 wide ( = 800 nS at 100 MHz VCLK3) Ah 11 x VCLK3 / 8 wide ( = 880 nS at 100 MHz VCLK3) Bh 12 x VCLK3 / 8 wide ( = 960 nS at 100 MHz VCLK3) Ch 13 x VCLK3 / 8 wide ( = 1040 nS at 100 MHz VCLK3) Dh 14 x VCLK3 / 8 wide ( = 1120 nS at 100 MHz VCLK3) Eh 15 x VCLK3 / 8 wide ( = 1200 nS at 100 MHz VCLK3) Fh 16 x VCLK3 / 8 wide ( = 1280 nS at 100 MHz VCLK3)"]
    #[inline(always)]
    pub fn oshtwth(&self) -> OshtwthR {
        OshtwthR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Chopping Clock Frequency 0 Divide by 1 (no prescale, = 12.5 MHz at 100 MHz VCLK3) 1h Divide by 2 (6.25 MHz at 100 MHz VCLK3) 2h Divide by 3 (4.16 MHz at 100 MHz VCLK3) 3h Divide by 4 (3.12 MHz at 100 MHz VCLK3) 4h Divide by 5 (2.50 MHz at 100 MHz VCLK3) 5h Divide by 6 (2.08 MHz at 100 MHz VCLK3) 6h Divide by 7 (1.78 MHz at 100 MHz VCLK3) 7h Divide by 8 (1.56 MHz at 100 MHz VCLK3)"]
    #[inline(always)]
    pub fn chpfreq(&self) -> ChpfreqR {
        ChpfreqR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Chopping Clock Duty Cycle 0 Duty = 1/8 (12.5%) 1h Duty = 2/8 (25.0%) 2h Duty = 3/8 (37.5%) 3h Duty = 4/8 (50.0%) 4h Duty = 5/8 (62.5%) 5h Duty = 6/8 (75.0%) 6h Duty = 7/8 (87.5%) 7h Reserved"]
    #[inline(always)]
    pub fn chpduty(&self) -> ChpdutyR {
        ChpdutyR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PWM-chopping Enable 0 Disable (bypass) PWM chopping function 1 Enable chopping function"]
    #[inline(always)]
    #[must_use]
    pub fn chpen(&mut self) -> ChpenW<PcctlSpec> {
        ChpenW::new(self, 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
One-Shot Pulse Width 0 1 x VCLK3 / 8 wide ( = 80 nS at 100 MHz VCLK3) 1h 2 x VCLK3 / 8 wide ( = 160 nS at 100 MHz VCLK3) 2h 3 x VCLK3 / 8 wide ( = 240 nS at 100 MHz VCLK3) 3h 4 x VCLK3 / 8 wide ( = 320 nS at 100 MHz VCLK3) 4h 5 x VCLK3 / 8 wide ( = 400 nS at 100 MHz VCLK3) 5h 6 x VCLK3 / 8 wide ( = 480 nS at 100 MHz VCLK3) 6h 7 x VCLK3 / 8 wide ( = 560 nS at 100 MHz VCLK3) 7h 8 x VCLK3 / 8 wide ( = 640 nS at 100 MHz VCLK3) 8h 9 x VCLK3 / 8 wide ( = 720 nS at 100 MHz VCLK3) 9h 10 x VCLK3 / 8 wide ( = 800 nS at 100 MHz VCLK3) Ah 11 x VCLK3 / 8 wide ( = 880 nS at 100 MHz VCLK3) Bh 12 x VCLK3 / 8 wide ( = 960 nS at 100 MHz VCLK3) Ch 13 x VCLK3 / 8 wide ( = 1040 nS at 100 MHz VCLK3) Dh 14 x VCLK3 / 8 wide ( = 1120 nS at 100 MHz VCLK3) Eh 15 x VCLK3 / 8 wide ( = 1200 nS at 100 MHz VCLK3) Fh 16 x VCLK3 / 8 wide ( = 1280 nS at 100 MHz VCLK3)"]
    #[inline(always)]
    #[must_use]
    pub fn oshtwth(&mut self) -> OshtwthW<PcctlSpec> {
        OshtwthW::new(self, 1)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Chopping Clock Frequency 0 Divide by 1 (no prescale, = 12.5 MHz at 100 MHz VCLK3) 1h Divide by 2 (6.25 MHz at 100 MHz VCLK3) 2h Divide by 3 (4.16 MHz at 100 MHz VCLK3) 3h Divide by 4 (3.12 MHz at 100 MHz VCLK3) 4h Divide by 5 (2.50 MHz at 100 MHz VCLK3) 5h Divide by 6 (2.08 MHz at 100 MHz VCLK3) 6h Divide by 7 (1.78 MHz at 100 MHz VCLK3) 7h Divide by 8 (1.56 MHz at 100 MHz VCLK3)"]
    #[inline(always)]
    #[must_use]
    pub fn chpfreq(&mut self) -> ChpfreqW<PcctlSpec> {
        ChpfreqW::new(self, 5)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Chopping Clock Duty Cycle 0 Duty = 1/8 (12.5%) 1h Duty = 2/8 (25.0%) 2h Duty = 3/8 (37.5%) 3h Duty = 4/8 (50.0%) 4h Duty = 5/8 (62.5%) 5h Duty = 6/8 (75.0%) 6h Duty = 7/8 (87.5%) 7h Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn chpduty(&mut self) -> ChpdutyW<PcctlSpec> {
        ChpdutyW::new(self, 8)
    }
}
#[doc = "PWM-Chopper Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcctlSpec;
impl crate::RegisterSpec for PcctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcctl::R`](R) reader structure"]
impl crate::Readable for PcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcctl::W`](W) writer structure"]
impl crate::Writable for PcctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCCTL to value 0"]
impl crate::Resettable for PcctlSpec {
    const RESET_VALUE: u32 = 0;
}
