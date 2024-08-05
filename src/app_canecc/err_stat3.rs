#[doc = "Register `ERR_STAT3` reader"]
pub type R = crate::R<ErrStat3Spec>;
#[doc = "Register `ERR_STAT3` writer"]
pub type W = crate::W<ErrStat3Spec>;
#[doc = "Field `NU4` reader - 0:0\\]
TI Internal : Reserved"]
pub type Nu4R = crate::BitReader;
#[doc = "Field `NU4` writer - 0:0\\]
TI Internal : Reserved"]
pub type Nu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_PEND` reader - 1:1\\]
TI Internal : Timeout pending"]
pub type TimeoutPendR = crate::BitReader;
#[doc = "Field `TIMEOUT_PEND` writer - 1:1\\]
TI Internal : Timeout pending"]
pub type TimeoutPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU5` reader - 8:2\\]
TI Internal : Reserved"]
pub type Nu5R = crate::FieldReader;
#[doc = "Field `NU5` writer - 8:2\\]
TI Internal : Reserved"]
pub type Nu5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CLR_TIMEOUT_PEND` reader - 9:9\\]
TI Internal : Clear timeout pending"]
pub type ClrTimeoutPendR = crate::BitReader;
#[doc = "Field `CLR_TIMEOUT_PEND` writer - 9:9\\]
TI Internal : Clear timeout pending"]
pub type ClrTimeoutPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU6` reader - 31:10\\]
TI Internal : Reserved"]
pub type Nu6R = crate::FieldReader<u32>;
#[doc = "Field `NU6` writer - 31:10\\]
TI Internal : Reserved"]
pub type Nu6W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TI Internal : Reserved"]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal : Timeout pending"]
    #[inline(always)]
    pub fn timeout_pend(&self) -> TimeoutPendR {
        TimeoutPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:8 - 8:2\\]
TI Internal : Reserved"]
    #[inline(always)]
    pub fn nu5(&self) -> Nu5R {
        Nu5R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
TI Internal : Clear timeout pending"]
    #[inline(always)]
    pub fn clr_timeout_pend(&self) -> ClrTimeoutPendR {
        ClrTimeoutPendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
TI Internal : Reserved"]
    #[inline(always)]
    pub fn nu6(&self) -> Nu6R {
        Nu6R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TI Internal : Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<ErrStat3Spec> {
        Nu4W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal : Timeout pending"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_pend(&mut self) -> TimeoutPendW<ErrStat3Spec> {
        TimeoutPendW::new(self, 1)
    }
    #[doc = "Bits 2:8 - 8:2\\]
TI Internal : Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu5(&mut self) -> Nu5W<ErrStat3Spec> {
        Nu5W::new(self, 2)
    }
    #[doc = "Bit 9 - 9:9\\]
TI Internal : Clear timeout pending"]
    #[inline(always)]
    #[must_use]
    pub fn clr_timeout_pend(&mut self) -> ClrTimeoutPendW<ErrStat3Spec> {
        ClrTimeoutPendW::new(self, 9)
    }
    #[doc = "Bits 10:31 - 31:10\\]
TI Internal : Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu6(&mut self) -> Nu6W<ErrStat3Spec> {
        Nu6W::new(self, 10)
    }
}
#[doc = "ERR_STAT3\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stat3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stat3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrStat3Spec;
impl crate::RegisterSpec for ErrStat3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stat3::R`](R) reader structure"]
impl crate::Readable for ErrStat3Spec {}
#[doc = "`write(|w| ..)` method takes [`err_stat3::W`](W) writer structure"]
impl crate::Writable for ErrStat3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_STAT3 to value 0"]
impl crate::Resettable for ErrStat3Spec {
    const RESET_VALUE: u32 = 0;
}
