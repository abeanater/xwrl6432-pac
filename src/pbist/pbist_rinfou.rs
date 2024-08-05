#[doc = "Register `PBIST_RINFOU` reader"]
pub type R = crate::R<PbistRinfouSpec>;
#[doc = "Register `PBIST_RINFOU` writer"]
pub type W = crate::W<PbistRinfouSpec>;
#[doc = "Field `RINFOU0` reader - 7:0\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfou0R = crate::FieldReader;
#[doc = "Field `RINFOU0` writer - 7:0\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfou0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RINFOU1` reader - 15:8\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfou1R = crate::FieldReader;
#[doc = "Field `RINFOU1` writer - 15:8\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfou1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RINFOU2` reader - 23:16\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfou2R = crate::FieldReader;
#[doc = "Field `RINFOU2` writer - 23:16\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfou2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RINFOU3` reader - 31:24\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfou3R = crate::FieldReader;
#[doc = "Field `RINFOU3` writer - 31:24\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfou3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    pub fn rinfou0(&self) -> Rinfou0R {
        Rinfou0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    pub fn rinfou1(&self) -> Rinfou1R {
        Rinfou1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    pub fn rinfou2(&self) -> Rinfou2R {
        Rinfou2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    pub fn rinfou3(&self) -> Rinfou3R {
        Rinfou3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    #[must_use]
    pub fn rinfou0(&mut self) -> Rinfou0W<PbistRinfouSpec> {
        Rinfou0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    #[must_use]
    pub fn rinfou1(&mut self) -> Rinfou1W<PbistRinfouSpec> {
        Rinfou1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    #[must_use]
    pub fn rinfou2(&mut self) -> Rinfou2W<PbistRinfouSpec> {
        Rinfou2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    #[must_use]
    pub fn rinfou3(&mut self) -> Rinfou3W<PbistRinfouSpec> {
        Rinfou3W::new(self, 24)
    }
}
#[doc = "RAM Info Mask Upper 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_rinfou::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_rinfou::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistRinfouSpec;
impl crate::RegisterSpec for PbistRinfouSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_rinfou::R`](R) reader structure"]
impl crate::Readable for PbistRinfouSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_rinfou::W`](W) writer structure"]
impl crate::Writable for PbistRinfouSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_RINFOU to value 0"]
impl crate::Resettable for PbistRinfouSpec {
    const RESET_VALUE: u32 = 0;
}
