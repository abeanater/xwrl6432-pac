#[doc = "Register `XIDFC` reader"]
pub type R = crate::R<XidfcSpec>;
#[doc = "Register `XIDFC` writer"]
pub type W = crate::W<XidfcSpec>;
#[doc = "Field `NU37` reader - 1:0\\]
Reserved"]
pub type Nu37R = crate::FieldReader;
#[doc = "Field `NU37` writer - 1:0\\]
Reserved"]
pub type Nu37W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLSSA_X` reader - 15:2\\]
Filter List Standard Start Address"]
pub type FlssaXR = crate::FieldReader<u16>;
#[doc = "Field `FLSSA_X` writer - 15:2\\]
Filter List Standard Start Address"]
pub type FlssaXW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSS_X` reader - 22:16\\]
List Size Standard"]
pub type LssXR = crate::FieldReader;
#[doc = "Field `LSS_X` writer - 22:16\\]
List Size Standard"]
pub type LssXW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU38` reader - 31:23\\]
Reserved"]
pub type Nu38R = crate::FieldReader<u16>;
#[doc = "Field `NU38` writer - 31:23\\]
Reserved"]
pub type Nu38W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu37(&self) -> Nu37R {
        Nu37R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa_x(&self) -> FlssaXR {
        FlssaXR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
List Size Standard"]
    #[inline(always)]
    pub fn lss_x(&self) -> LssXR {
        LssXR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    pub fn nu38(&self) -> Nu38R {
        Nu38R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu37(&mut self) -> Nu37W<XidfcSpec> {
        Nu37W::new(self, 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Filter List Standard Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flssa_x(&mut self) -> FlssaXW<XidfcSpec> {
        FlssaXW::new(self, 2)
    }
    #[doc = "Bits 16:22 - 22:16\\]
List Size Standard"]
    #[inline(always)]
    #[must_use]
    pub fn lss_x(&mut self) -> LssXW<XidfcSpec> {
        LssXW::new(self, 16)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu38(&mut self) -> Nu38W<XidfcSpec> {
        Nu38W::new(self, 23)
    }
}
#[doc = "XIDFC\n\nYou can [`read`](crate::Reg::read) this register and get [`xidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XidfcSpec;
impl crate::RegisterSpec for XidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidfc::R`](R) reader structure"]
impl crate::Readable for XidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`xidfc::W`](W) writer structure"]
impl crate::Writable for XidfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIDFC to value 0"]
impl crate::Resettable for XidfcSpec {
    const RESET_VALUE: u32 = 0;
}
