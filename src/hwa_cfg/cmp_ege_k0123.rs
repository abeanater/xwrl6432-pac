#[doc = "Register `CMP_EGE_K0123` reader"]
pub type R = crate::R<CmpEgeK0123Spec>;
#[doc = "Register `CMP_EGE_K0123` writer"]
pub type W = crate::W<CmpEgeK0123Spec>;
#[doc = "Field `CMP_EGE_K0` reader - 4:0\\]
EGE K-param for the 1st accumulator"]
pub type CmpEgeK0R = crate::FieldReader;
#[doc = "Field `CMP_EGE_K0` writer - 4:0\\]
EGE K-param for the 1st accumulator"]
pub type CmpEgeK0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU1` reader - 7:5\\]
Reserved.TI internal"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:5\\]
Reserved.TI internal"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP_EGE_K1` reader - 12:8\\]
EGE K-param for the 2nd accumulator"]
pub type CmpEgeK1R = crate::FieldReader;
#[doc = "Field `CMP_EGE_K1` writer - 12:8\\]
EGE K-param for the 2nd accumulator"]
pub type CmpEgeK1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU2` reader - 15:13\\]
Reserved.TI internal"]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 15:13\\]
Reserved.TI internal"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP_EGE_K2` reader - 20:16\\]
EGE K-param for the 3rd accumulator"]
pub type CmpEgeK2R = crate::FieldReader;
#[doc = "Field `CMP_EGE_K2` writer - 20:16\\]
EGE K-param for the 3rd accumulator"]
pub type CmpEgeK2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU3` reader - 23:21\\]
Reserved.TI internal"]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 23:21\\]
Reserved.TI internal"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP_EGE_K3` reader - 28:24\\]
EGE K-param for the 4th accumulator"]
pub type CmpEgeK3R = crate::FieldReader;
#[doc = "Field `CMP_EGE_K3` writer - 28:24\\]
EGE K-param for the 4th accumulator"]
pub type CmpEgeK3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU4` reader - 31:29\\]
Reserved.TI internal"]
pub type Nu4R = crate::FieldReader;
#[doc = "Field `NU4` writer - 31:29\\]
Reserved.TI internal"]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
EGE K-param for the 1st accumulator"]
    #[inline(always)]
    pub fn cmp_ege_k0(&self) -> CmpEgeK0R {
        CmpEgeK0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
EGE K-param for the 2nd accumulator"]
    #[inline(always)]
    pub fn cmp_ege_k1(&self) -> CmpEgeK1R {
        CmpEgeK1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
EGE K-param for the 3rd accumulator"]
    #[inline(always)]
    pub fn cmp_ege_k2(&self) -> CmpEgeK2R {
        CmpEgeK2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
EGE K-param for the 4th accumulator"]
    #[inline(always)]
    pub fn cmp_ege_k3(&self) -> CmpEgeK3R {
        CmpEgeK3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
EGE K-param for the 1st accumulator"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ege_k0(&mut self) -> CmpEgeK0W<CmpEgeK0123Spec> {
        CmpEgeK0W::new(self, 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<CmpEgeK0123Spec> {
        Nu1W::new(self, 5)
    }
    #[doc = "Bits 8:12 - 12:8\\]
EGE K-param for the 2nd accumulator"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ege_k1(&mut self) -> CmpEgeK1W<CmpEgeK0123Spec> {
        CmpEgeK1W::new(self, 8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<CmpEgeK0123Spec> {
        Nu2W::new(self, 13)
    }
    #[doc = "Bits 16:20 - 20:16\\]
EGE K-param for the 3rd accumulator"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ege_k2(&mut self) -> CmpEgeK2W<CmpEgeK0123Spec> {
        CmpEgeK2W::new(self, 16)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<CmpEgeK0123Spec> {
        Nu3W::new(self, 21)
    }
    #[doc = "Bits 24:28 - 28:24\\]
EGE K-param for the 4th accumulator"]
    #[inline(always)]
    #[must_use]
    pub fn cmp_ege_k3(&mut self) -> CmpEgeK3W<CmpEgeK0123Spec> {
        CmpEgeK3W::new(self, 24)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<CmpEgeK0123Spec> {
        Nu4W::new(self, 29)
    }
}
#[doc = "CMP_EGE_K0123\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ege_k0123::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ege_k0123::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpEgeK0123Spec;
impl crate::RegisterSpec for CmpEgeK0123Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_ege_k0123::R`](R) reader structure"]
impl crate::Readable for CmpEgeK0123Spec {}
#[doc = "`write(|w| ..)` method takes [`cmp_ege_k0123::W`](W) writer structure"]
impl crate::Writable for CmpEgeK0123Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP_EGE_K0123 to value 0"]
impl crate::Resettable for CmpEgeK0123Spec {
    const RESET_VALUE: u32 = 0;
}
