#[doc = "Register `GIOINTDET` reader"]
pub type R = crate::R<GiointdetSpec>;
#[doc = "Register `GIOINTDET` writer"]
pub type W = crate::W<GiointdetSpec>;
#[doc = "Field `GIOINTDET_0` reader - 7:0\\]
Interrupt detection select for pins GIOA\\[7:0\\]."]
pub type Giointdet0R = crate::FieldReader;
#[doc = "Field `GIOINTDET_0` writer - 7:0\\]
Interrupt detection select for pins GIOA\\[7:0\\]."]
pub type Giointdet0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOINTDET_1` reader - 15:8\\]
Interrupt detection select for pins GIOB\\[7:0\\]."]
pub type Giointdet1R = crate::FieldReader;
#[doc = "Field `GIOINTDET_1` writer - 15:8\\]
Interrupt detection select for pins GIOB\\[7:0\\]."]
pub type Giointdet1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOINTDET_2` reader - 23:16\\]
Interrupt detection select for pins GIOC\\[7:0\\]."]
pub type Giointdet2R = crate::FieldReader;
#[doc = "Field `GIOINTDET_2` writer - 23:16\\]
Interrupt detection select for pins GIOC\\[7:0\\]."]
pub type Giointdet2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GIOINTDET_3` reader - 31:24\\]
Interrupt detection select for pins GIOD\\[7:0\\]."]
pub type Giointdet3R = crate::FieldReader;
#[doc = "Field `GIOINTDET_3` writer - 31:24\\]
Interrupt detection select for pins GIOD\\[7:0\\]."]
pub type Giointdet3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt detection select for pins GIOA\\[7:0\\]."]
    #[inline(always)]
    pub fn giointdet_0(&self) -> Giointdet0R {
        Giointdet0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Interrupt detection select for pins GIOB\\[7:0\\]."]
    #[inline(always)]
    pub fn giointdet_1(&self) -> Giointdet1R {
        Giointdet1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Interrupt detection select for pins GIOC\\[7:0\\]."]
    #[inline(always)]
    pub fn giointdet_2(&self) -> Giointdet2R {
        Giointdet2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Interrupt detection select for pins GIOD\\[7:0\\]."]
    #[inline(always)]
    pub fn giointdet_3(&self) -> Giointdet3R {
        Giointdet3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt detection select for pins GIOA\\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn giointdet_0(&mut self) -> Giointdet0W<GiointdetSpec> {
        Giointdet0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Interrupt detection select for pins GIOB\\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn giointdet_1(&mut self) -> Giointdet1W<GiointdetSpec> {
        Giointdet1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Interrupt detection select for pins GIOC\\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn giointdet_2(&mut self) -> Giointdet2W<GiointdetSpec> {
        Giointdet2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Interrupt detection select for pins GIOD\\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn giointdet_3(&mut self) -> Giointdet3W<GiointdetSpec> {
        Giointdet3W::new(self, 24)
    }
}
#[doc = "Interrupt detection select for pins \\[0:1\\]
GIO\\[7:0\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`giointdet::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giointdet::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiointdetSpec;
impl crate::RegisterSpec for GiointdetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giointdet::R`](R) reader structure"]
impl crate::Readable for GiointdetSpec {}
#[doc = "`write(|w| ..)` method takes [`giointdet::W`](W) writer structure"]
impl crate::Writable for GiointdetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOINTDET to value 0"]
impl crate::Resettable for GiointdetSpec {
    const RESET_VALUE: u32 = 0;
}
