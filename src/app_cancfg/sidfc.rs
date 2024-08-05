#[doc = "Register `SIDFC` reader"]
pub type R = crate::R<SidfcSpec>;
#[doc = "Register `SIDFC` writer"]
pub type W = crate::W<SidfcSpec>;
#[doc = "Field `NU35` reader - 1:0\\]
Reserved"]
pub type Nu35R = crate::FieldReader;
#[doc = "Field `NU35` writer - 1:0\\]
Reserved"]
pub type Nu35W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLSSA_S` reader - 15:2\\]
Filter List Standard Start Address"]
pub type FlssaSR = crate::FieldReader<u16>;
#[doc = "Field `FLSSA_S` writer - 15:2\\]
Filter List Standard Start Address"]
pub type FlssaSW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSS_S` reader - 23:16\\]
List Size Standard"]
pub type LssSR = crate::FieldReader;
#[doc = "Field `LSS_S` writer - 23:16\\]
List Size Standard"]
pub type LssSW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU36` reader - 31:24\\]
Reserved"]
pub type Nu36R = crate::FieldReader;
#[doc = "Field `NU36` writer - 31:24\\]
Reserved"]
pub type Nu36W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu35(&self) -> Nu35R {
        Nu35R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa_s(&self) -> FlssaSR {
        FlssaSR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
List Size Standard"]
    #[inline(always)]
    pub fn lss_s(&self) -> LssSR {
        LssSR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    pub fn nu36(&self) -> Nu36R {
        Nu36R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu35(&mut self) -> Nu35W<SidfcSpec> {
        Nu35W::new(self, 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Filter List Standard Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn flssa_s(&mut self) -> FlssaSW<SidfcSpec> {
        FlssaSW::new(self, 2)
    }
    #[doc = "Bits 16:23 - 23:16\\]
List Size Standard"]
    #[inline(always)]
    #[must_use]
    pub fn lss_s(&mut self) -> LssSW<SidfcSpec> {
        LssSW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu36(&mut self) -> Nu36W<SidfcSpec> {
        Nu36W::new(self, 24)
    }
}
#[doc = "SIDFC\n\nYou can [`read`](crate::Reg::read) this register and get [`sidfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sidfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidfcSpec;
impl crate::RegisterSpec for SidfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidfc::R`](R) reader structure"]
impl crate::Readable for SidfcSpec {}
#[doc = "`write(|w| ..)` method takes [`sidfc::W`](W) writer structure"]
impl crate::Writable for SidfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIDFC to value 0"]
impl crate::Resettable for SidfcSpec {
    const RESET_VALUE: u32 = 0;
}
