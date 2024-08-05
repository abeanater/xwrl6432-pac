#[doc = "Register `QUETCMAP` reader"]
pub type R = crate::R<QuetcmapSpec>;
#[doc = "Register `QUETCMAP` writer"]
pub type W = crate::W<QuetcmapSpec>;
#[doc = "Field `TCNUMQ0` reader - 2:0\\]
TC Number for Queue N: Defines the TC number that Event Queue N TRs are written to."]
pub type Tcnumq0R = crate::FieldReader;
#[doc = "Field `TCNUMQ0` writer - 2:0\\]
TC Number for Queue N: Defines the TC number that Event Queue N TRs are written to."]
pub type Tcnumq0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES28` reader - 3:3\\]
RESERVE FIELD"]
pub type Res28R = crate::BitReader;
#[doc = "Field `RES28` writer - 3:3\\]
RESERVE FIELD"]
pub type Res28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCNUMQ1` reader - 6:4\\]
TC Number for Queue N: Defines the TC number that Event Queue N TRs are written to."]
pub type Tcnumq1R = crate::FieldReader;
#[doc = "Field `TCNUMQ1` writer - 6:4\\]
TC Number for Queue N: Defines the TC number that Event Queue N TRs are written to."]
pub type Tcnumq1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES27` reader - 31:7\\]
RESERVE FIELD"]
pub type Res27R = crate::FieldReader<u32>;
#[doc = "Field `RES27` writer - 31:7\\]
RESERVE FIELD"]
pub type Res27W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
TC Number for Queue N: Defines the TC number that Event Queue N TRs are written to."]
    #[inline(always)]
    pub fn tcnumq0(&self) -> Tcnumq0R {
        Tcnumq0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res28(&self) -> Res28R {
        Res28R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
TC Number for Queue N: Defines the TC number that Event Queue N TRs are written to."]
    #[inline(always)]
    pub fn tcnumq1(&self) -> Tcnumq1R {
        Tcnumq1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:31 - 31:7\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res27(&self) -> Res27R {
        Res27R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
TC Number for Queue N: Defines the TC number that Event Queue N TRs are written to."]
    #[inline(always)]
    #[must_use]
    pub fn tcnumq0(&mut self) -> Tcnumq0W<QuetcmapSpec> {
        Tcnumq0W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res28(&mut self) -> Res28W<QuetcmapSpec> {
        Res28W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
TC Number for Queue N: Defines the TC number that Event Queue N TRs are written to."]
    #[inline(always)]
    #[must_use]
    pub fn tcnumq1(&mut self) -> Tcnumq1W<QuetcmapSpec> {
        Tcnumq1W::new(self, 4)
    }
    #[doc = "Bits 7:31 - 31:7\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res27(&mut self) -> Res27W<QuetcmapSpec> {
        Res27W::new(self, 7)
    }
}
#[doc = "Queue to TC Mapping\n\nYou can [`read`](crate::Reg::read) this register and get [`quetcmap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`quetcmap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QuetcmapSpec;
impl crate::RegisterSpec for QuetcmapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quetcmap::R`](R) reader structure"]
impl crate::Readable for QuetcmapSpec {}
#[doc = "`write(|w| ..)` method takes [`quetcmap::W`](W) writer structure"]
impl crate::Writable for QuetcmapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUETCMAP to value 0"]
impl crate::Resettable for QuetcmapSpec {
    const RESET_VALUE: u32 = 0;
}
