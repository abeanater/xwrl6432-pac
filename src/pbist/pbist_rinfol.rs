#[doc = "Register `PBIST_RINFOL` reader"]
pub type R = crate::R<PbistRinfolSpec>;
#[doc = "Register `PBIST_RINFOL` writer"]
pub type W = crate::W<PbistRinfolSpec>;
#[doc = "Field `RINFOL0` reader - 7:0\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfol0R = crate::FieldReader;
#[doc = "Field `RINFOL0` writer - 7:0\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfol0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RINFOL1` reader - 15:8\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfol1R = crate::FieldReader;
#[doc = "Field `RINFOL1` writer - 15:8\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfol1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RINFOL2` reader - 23:16\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfol2R = crate::FieldReader;
#[doc = "Field `RINFOL2` writer - 23:16\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfol2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RINFOL3` reader - 31:24\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfol3R = crate::FieldReader;
#[doc = "Field `RINFOL3` writer - 31:24\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
pub type Rinfol3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    pub fn rinfol0(&self) -> Rinfol0R {
        Rinfol0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    pub fn rinfol1(&self) -> Rinfol1R {
        Rinfol1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    pub fn rinfol2(&self) -> Rinfol2R {
        Rinfol2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    pub fn rinfol3(&self) -> Rinfol3R {
        Rinfol3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    #[must_use]
    pub fn rinfol0(&mut self) -> Rinfol0W<PbistRinfolSpec> {
        Rinfol0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    #[must_use]
    pub fn rinfol1(&mut self) -> Rinfol1W<PbistRinfolSpec> {
        Rinfol1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    #[must_use]
    pub fn rinfol2(&mut self) -> Rinfol2W<PbistRinfolSpec> {
        Rinfol2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This register is to select memory groups to run the algorithms selected in the PBIST_ALGO register. For an algorithmto be executed on a particular memory group, the corresponding bit in this register must be set to 1. The default value of this register is all 1s, which means all the memory groups are selected. Writing a value 0 to the particular bit, disables the corresponding memory group."]
    #[inline(always)]
    #[must_use]
    pub fn rinfol3(&mut self) -> Rinfol3W<PbistRinfolSpec> {
        Rinfol3W::new(self, 24)
    }
}
#[doc = "RAM Info Mask Lower 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_rinfol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_rinfol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistRinfolSpec;
impl crate::RegisterSpec for PbistRinfolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_rinfol::R`](R) reader structure"]
impl crate::Readable for PbistRinfolSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_rinfol::W`](W) writer structure"]
impl crate::Writable for PbistRinfolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_RINFOL to value 0"]
impl crate::Resettable for PbistRinfolSpec {
    const RESET_VALUE: u32 = 0;
}
