#[doc = "Register `TSCV` reader"]
pub type R = crate::R<TscvSpec>;
#[doc = "Register `TSCV` writer"]
pub type W = crate::W<TscvSpec>;
#[doc = "Field `TSC` reader - 15:0\\]
Timestamp Counter"]
pub type TscR = crate::FieldReader<u16>;
#[doc = "Field `TSC` writer - 15:0\\]
Timestamp Counter"]
pub type TscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU22` reader - 31:16\\]
Reserved"]
pub type Nu22R = crate::FieldReader<u16>;
#[doc = "Field `NU22` writer - 31:16\\]
Reserved"]
pub type Nu22W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timestamp Counter"]
    #[inline(always)]
    pub fn tsc(&self) -> TscR {
        TscR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu22(&self) -> Nu22R {
        Nu22R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Timestamp Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tsc(&mut self) -> TscW<TscvSpec> {
        TscW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu22(&mut self) -> Nu22W<TscvSpec> {
        Nu22W::new(self, 16)
    }
}
#[doc = "TSCV\n\nYou can [`read`](crate::Reg::read) this register and get [`tscv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TscvSpec;
impl crate::RegisterSpec for TscvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscv::R`](R) reader structure"]
impl crate::Readable for TscvSpec {}
#[doc = "`write(|w| ..)` method takes [`tscv::W`](W) writer structure"]
impl crate::Writable for TscvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TscvSpec {
    const RESET_VALUE: u32 = 0;
}
