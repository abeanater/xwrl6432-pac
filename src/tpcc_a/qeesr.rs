#[doc = "Register `QEESR` reader"]
pub type R = crate::R<QeesrSpec>;
#[doc = "Register `QEESR` writer"]
pub type W = crate::W<QeesrSpec>;
#[doc = "Field `E0` reader - 0:0\\]
Event #0"]
pub type E0R = crate::BitReader;
#[doc = "Field `E0` writer - 0:0\\]
Event #0"]
pub type E0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E1` reader - 1:1\\]
Event #1"]
pub type E1R = crate::BitReader;
#[doc = "Field `E1` writer - 1:1\\]
Event #1"]
pub type E1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E2` reader - 2:2\\]
Event #2"]
pub type E2R = crate::BitReader;
#[doc = "Field `E2` writer - 2:2\\]
Event #2"]
pub type E2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E3` reader - 3:3\\]
Event #3"]
pub type E3R = crate::BitReader;
#[doc = "Field `E3` writer - 3:3\\]
Event #3"]
pub type E3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E4` reader - 4:4\\]
Event #4"]
pub type E4R = crate::BitReader;
#[doc = "Field `E4` writer - 4:4\\]
Event #4"]
pub type E4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E5` reader - 5:5\\]
Event #5"]
pub type E5R = crate::BitReader;
#[doc = "Field `E5` writer - 5:5\\]
Event #5"]
pub type E5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E6` reader - 6:6\\]
Event #6"]
pub type E6R = crate::BitReader;
#[doc = "Field `E6` writer - 6:6\\]
Event #6"]
pub type E6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E7` reader - 7:7\\]
Event #7"]
pub type E7R = crate::BitReader;
#[doc = "Field `E7` writer - 7:7\\]
Event #7"]
pub type E7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES73` reader - 31:8\\]
RESERVE FIELD"]
pub type Res73R = crate::FieldReader<u32>;
#[doc = "Field `RES73` writer - 31:8\\]
RESERVE FIELD"]
pub type Res73W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Event #0"]
    #[inline(always)]
    pub fn e0(&self) -> E0R {
        E0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Event #1"]
    #[inline(always)]
    pub fn e1(&self) -> E1R {
        E1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Event #2"]
    #[inline(always)]
    pub fn e2(&self) -> E2R {
        E2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Event #3"]
    #[inline(always)]
    pub fn e3(&self) -> E3R {
        E3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Event #4"]
    #[inline(always)]
    pub fn e4(&self) -> E4R {
        E4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Event #5"]
    #[inline(always)]
    pub fn e5(&self) -> E5R {
        E5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Event #6"]
    #[inline(always)]
    pub fn e6(&self) -> E6R {
        E6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Event #7"]
    #[inline(always)]
    pub fn e7(&self) -> E7R {
        E7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res73(&self) -> Res73R {
        Res73R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Event #0"]
    #[inline(always)]
    #[must_use]
    pub fn e0(&mut self) -> E0W<QeesrSpec> {
        E0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Event #1"]
    #[inline(always)]
    #[must_use]
    pub fn e1(&mut self) -> E1W<QeesrSpec> {
        E1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Event #2"]
    #[inline(always)]
    #[must_use]
    pub fn e2(&mut self) -> E2W<QeesrSpec> {
        E2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Event #3"]
    #[inline(always)]
    #[must_use]
    pub fn e3(&mut self) -> E3W<QeesrSpec> {
        E3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Event #4"]
    #[inline(always)]
    #[must_use]
    pub fn e4(&mut self) -> E4W<QeesrSpec> {
        E4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Event #5"]
    #[inline(always)]
    #[must_use]
    pub fn e5(&mut self) -> E5W<QeesrSpec> {
        E5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Event #6"]
    #[inline(always)]
    #[must_use]
    pub fn e6(&mut self) -> E6W<QeesrSpec> {
        E6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Event #7"]
    #[inline(always)]
    #[must_use]
    pub fn e7(&mut self) -> E7W<QeesrSpec> {
        E7W::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res73(&mut self) -> Res73W<QeesrSpec> {
        Res73W::new(self, 8)
    }
}
#[doc = "QDMA Event Enable Set Register: CPU write of '1' to the QEESR.En bit causes the QEESR.En bit to be set. CPU write of '0' has no effect..\n\nYou can [`read`](crate::Reg::read) this register and get [`qeesr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qeesr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QeesrSpec;
impl crate::RegisterSpec for QeesrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qeesr::R`](R) reader structure"]
impl crate::Readable for QeesrSpec {}
#[doc = "`write(|w| ..)` method takes [`qeesr::W`](W) writer structure"]
impl crate::Writable for QeesrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QEESR to value 0"]
impl crate::Resettable for QeesrSpec {
    const RESET_VALUE: u32 = 0;
}
