#[doc = "Register `GIOGCR` reader"]
pub type R = crate::R<GiogcrSpec>;
#[doc = "Register `GIOGCR` writer"]
pub type W = crate::W<GiogcrSpec>;
#[doc = "Field `RESET` reader - 0:0\\]
GIO reset"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - 0:0\\]
GIO reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU0` reader - 31:1\\]
Reserved"]
pub type Nu0R = crate::FieldReader<u32>;
#[doc = "Field `NU0` writer - 31:1\\]
Reserved"]
pub type Nu0W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
GIO reset"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn nu0(&self) -> Nu0R {
        Nu0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
GIO reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<GiogcrSpec> {
        ResetW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu0(&mut self) -> Nu0W<GiogcrSpec> {
        Nu0W::new(self, 1)
    }
}
#[doc = "GIO reset\n\nYou can [`read`](crate::Reg::read) this register and get [`giogcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giogcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiogcrSpec;
impl crate::RegisterSpec for GiogcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giogcr::R`](R) reader structure"]
impl crate::Readable for GiogcrSpec {}
#[doc = "`write(|w| ..)` method takes [`giogcr::W`](W) writer structure"]
impl crate::Writable for GiogcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOGCR to value 0"]
impl crate::Resettable for GiogcrSpec {
    const RESET_VALUE: u32 = 0;
}
