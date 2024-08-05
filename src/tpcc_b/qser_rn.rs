#[doc = "Register `QSER_RN` reader"]
pub type R = crate::R<QserRnSpec>;
#[doc = "Register `QSER_RN` writer"]
pub type W = crate::W<QserRnSpec>;
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
#[doc = "Field `RES81` reader - 31:8\\]
RESERVE FIELD"]
pub type Res81R = crate::FieldReader<u32>;
#[doc = "Field `RES81` writer - 31:8\\]
RESERVE FIELD"]
pub type Res81W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
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
    pub fn res81(&self) -> Res81R {
        Res81R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Event #0"]
    #[inline(always)]
    #[must_use]
    pub fn e0(&mut self) -> E0W<QserRnSpec> {
        E0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Event #1"]
    #[inline(always)]
    #[must_use]
    pub fn e1(&mut self) -> E1W<QserRnSpec> {
        E1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Event #2"]
    #[inline(always)]
    #[must_use]
    pub fn e2(&mut self) -> E2W<QserRnSpec> {
        E2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Event #3"]
    #[inline(always)]
    #[must_use]
    pub fn e3(&mut self) -> E3W<QserRnSpec> {
        E3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Event #4"]
    #[inline(always)]
    #[must_use]
    pub fn e4(&mut self) -> E4W<QserRnSpec> {
        E4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Event #5"]
    #[inline(always)]
    #[must_use]
    pub fn e5(&mut self) -> E5W<QserRnSpec> {
        E5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Event #6"]
    #[inline(always)]
    #[must_use]
    pub fn e6(&mut self) -> E6W<QserRnSpec> {
        E6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Event #7"]
    #[inline(always)]
    #[must_use]
    pub fn e7(&mut self) -> E7W<QserRnSpec> {
        E7W::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res81(&mut self) -> Res81W<QserRnSpec> {
        Res81W::new(self, 8)
    }
}
#[doc = "QDMA Secondary Event Register: The QDMA secondary event register is used along with the QDMA Event Register (QER) to provide information on the state of a QDMA Event. En = 0 : Event is not currently in the Event Queue. En = 1 : Event is currently stored in Event Queue. Event arbiter will not prioritize additional events.\n\nYou can [`read`](crate::Reg::read) this register and get [`qser_rn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qser_rn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QserRnSpec;
impl crate::RegisterSpec for QserRnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qser_rn::R`](R) reader structure"]
impl crate::Readable for QserRnSpec {}
#[doc = "`write(|w| ..)` method takes [`qser_rn::W`](W) writer structure"]
impl crate::Writable for QserRnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSER_RN to value 0"]
impl crate::Resettable for QserRnSpec {
    const RESET_VALUE: u32 = 0;
}
