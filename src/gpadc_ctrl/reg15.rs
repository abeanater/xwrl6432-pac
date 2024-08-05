#[doc = "Register `REG15` reader"]
pub type R = crate::R<Reg15Spec>;
#[doc = "Register `REG15` writer"]
pub type W = crate::W<Reg15Spec>;
#[doc = "Field `MIN_GPADC` reader - 9:0\\]
Min of GPADC readings"]
pub type MinGpadcR = crate::FieldReader<u16>;
#[doc = "Field `MIN_GPADC` writer - 9:0\\]
Min of GPADC readings"]
pub type MinGpadcW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NU1` reader - 15:10\\]
TI reserved"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 15:10\\]
TI reserved"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MAX_GPADC` reader - 25:16\\]
Max of GPADC readings"]
pub type MaxGpadcR = crate::FieldReader<u16>;
#[doc = "Field `MAX_GPADC` writer - 25:16\\]
Max of GPADC readings"]
pub type MaxGpadcW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NU2` reader - 31:26\\]
TI reserved"]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 31:26\\]
TI reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Min of GPADC readings"]
    #[inline(always)]
    pub fn min_gpadc(&self) -> MinGpadcR {
        MinGpadcR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - 15:10\\]
TI reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Max of GPADC readings"]
    #[inline(always)]
    pub fn max_gpadc(&self) -> MaxGpadcR {
        MaxGpadcR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:31 - 31:26\\]
TI reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Min of GPADC readings"]
    #[inline(always)]
    #[must_use]
    pub fn min_gpadc(&mut self) -> MinGpadcW<Reg15Spec> {
        MinGpadcW::new(self, 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Reg15Spec> {
        Nu1W::new(self, 10)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Max of GPADC readings"]
    #[inline(always)]
    #[must_use]
    pub fn max_gpadc(&mut self) -> MaxGpadcW<Reg15Spec> {
        MaxGpadcW::new(self, 16)
    }
    #[doc = "Bits 26:31 - 31:26\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Reg15Spec> {
        Nu2W::new(self, 26)
    }
}
#[doc = "Min and Max of GP ADC readings\n\nYou can [`read`](crate::Reg::read) this register and get [`reg15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg15Spec;
impl crate::RegisterSpec for Reg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg15::R`](R) reader structure"]
impl crate::Readable for Reg15Spec {}
#[doc = "`write(|w| ..)` method takes [`reg15::W`](W) writer structure"]
impl crate::Writable for Reg15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG15 to value 0"]
impl crate::Resettable for Reg15Spec {
    const RESET_VALUE: u32 = 0;
}
