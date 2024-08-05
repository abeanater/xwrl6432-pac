#[doc = "Register `TPTC_DBS_CONFIG` reader"]
pub type R = crate::R<TptcDbsConfigSpec>;
#[doc = "Register `TPTC_DBS_CONFIG` writer"]
pub type W = crate::W<TptcDbsConfigSpec>;
#[doc = "Field `tptc_a0` reader - 1:0\\]
Default burst size tieoff value for TPTC_A0"]
pub type TptcA0R = crate::FieldReader;
#[doc = "Field `tptc_a0` writer - 1:0\\]
Default burst size tieoff value for TPTC_A0"]
pub type TptcA0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_a1` reader - 5:4\\]
Default burst size tieoff value for TPTC_A1"]
pub type TptcA1R = crate::FieldReader;
#[doc = "Field `tptc_a1` writer - 5:4\\]
Default burst size tieoff value for TPTC_A1"]
pub type TptcA1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_b0` reader - 9:8\\]
Default burst size tieoff value for TPTC_B0"]
pub type TptcB0R = crate::FieldReader;
#[doc = "Field `tptc_b0` writer - 9:8\\]
Default burst size tieoff value for TPTC_B0"]
pub type TptcB0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tptc_b1` reader - 13:12\\]
Default burst size tieoff value for TPTC_B1"]
pub type TptcB1R = crate::FieldReader;
#[doc = "Field `tptc_b1` writer - 13:12\\]
Default burst size tieoff value for TPTC_B1"]
pub type TptcB1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Default burst size tieoff value for TPTC_A0"]
    #[inline(always)]
    pub fn tptc_a0(&self) -> TptcA0R {
        TptcA0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Default burst size tieoff value for TPTC_A1"]
    #[inline(always)]
    pub fn tptc_a1(&self) -> TptcA1R {
        TptcA1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Default burst size tieoff value for TPTC_B0"]
    #[inline(always)]
    pub fn tptc_b0(&self) -> TptcB0R {
        TptcB0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Default burst size tieoff value for TPTC_B1"]
    #[inline(always)]
    pub fn tptc_b1(&self) -> TptcB1R {
        TptcB1R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Default burst size tieoff value for TPTC_A0"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0(&mut self) -> TptcA0W<TptcDbsConfigSpec> {
        TptcA0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Default burst size tieoff value for TPTC_A1"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1(&mut self) -> TptcA1W<TptcDbsConfigSpec> {
        TptcA1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Default burst size tieoff value for TPTC_B0"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0(&mut self) -> TptcB0W<TptcDbsConfigSpec> {
        TptcB0W::new(self, 8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Default burst size tieoff value for TPTC_B1"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1(&mut self) -> TptcB1W<TptcDbsConfigSpec> {
        TptcB1W::new(self, 12)
    }
}
#[doc = "TPTC_DBS_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`tptc_dbs_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tptc_dbs_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TptcDbsConfigSpec;
impl crate::RegisterSpec for TptcDbsConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tptc_dbs_config::R`](R) reader structure"]
impl crate::Readable for TptcDbsConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`tptc_dbs_config::W`](W) writer structure"]
impl crate::Writable for TptcDbsConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPTC_DBS_CONFIG to value 0"]
impl crate::Resettable for TptcDbsConfigSpec {
    const RESET_VALUE: u32 = 0;
}
