#[doc = "Register `PLLDIG_STATUS` reader"]
pub type R = crate::R<PlldigStatusSpec>;
#[doc = "Register `PLLDIG_STATUS` writer"]
pub type W = crate::W<PlldigStatusSpec>;
#[doc = "Field `clkm_xtal_freq` reader - 1:0\\]
XTAL clock frequency status, 0x0 = 25MHz 0x1 = 40MHz 0x2 = 26MHz 0x3 = 38.4MHz"]
pub type ClkmXtalFreqR = crate::FieldReader;
#[doc = "Field `clkm_xtal_freq` writer - 1:0\\]
XTAL clock frequency status, 0x0 = 25MHz 0x1 = 40MHz 0x2 = 26MHz 0x3 = 38.4MHz"]
pub type ClkmXtalFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `plldig_lockmon` reader - 8:8\\]
PLL DIG lockmon status"]
pub type PlldigLockmonR = crate::BitReader;
#[doc = "Field `plldig_lockmon` writer - 8:8\\]
PLL DIG lockmon status"]
pub type PlldigLockmonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
XTAL clock frequency status, 0x0 = 25MHz 0x1 = 40MHz 0x2 = 26MHz 0x3 = 38.4MHz"]
    #[inline(always)]
    pub fn clkm_xtal_freq(&self) -> ClkmXtalFreqR {
        ClkmXtalFreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
PLL DIG lockmon status"]
    #[inline(always)]
    pub fn plldig_lockmon(&self) -> PlldigLockmonR {
        PlldigLockmonR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
XTAL clock frequency status, 0x0 = 25MHz 0x1 = 40MHz 0x2 = 26MHz 0x3 = 38.4MHz"]
    #[inline(always)]
    #[must_use]
    pub fn clkm_xtal_freq(&mut self) -> ClkmXtalFreqW<PlldigStatusSpec> {
        ClkmXtalFreqW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PLL DIG lockmon status"]
    #[inline(always)]
    #[must_use]
    pub fn plldig_lockmon(&mut self) -> PlldigLockmonW<PlldigStatusSpec> {
        PlldigLockmonW::new(self, 8)
    }
}
#[doc = "PLLDIG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldigStatusSpec;
impl crate::RegisterSpec for PlldigStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldig_status::R`](R) reader structure"]
impl crate::Readable for PlldigStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`plldig_status::W`](W) writer structure"]
impl crate::Writable for PlldigStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDIG_STATUS to value 0"]
impl crate::Resettable for PlldigStatusSpec {
    const RESET_VALUE: u32 = 0;
}
