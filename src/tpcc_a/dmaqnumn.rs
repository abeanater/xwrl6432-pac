#[doc = "Register `DMAQNUMN` reader"]
pub type R = crate::R<DmaqnumnSpec>;
#[doc = "Register `DMAQNUMN` writer"]
pub type W = crate::W<DmaqnumnSpec>;
#[doc = "Field `E0` reader - 2:0\\]
DMA Queue Number for event #0"]
pub type E0R = crate::FieldReader;
#[doc = "Field `E0` writer - 2:0\\]
DMA Queue Number for event #0"]
pub type E0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES18` reader - 3:3\\]
RESERVE FIELD"]
pub type Res18R = crate::BitReader;
#[doc = "Field `RES18` writer - 3:3\\]
RESERVE FIELD"]
pub type Res18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E1` reader - 6:4\\]
DMA Queue Number for event #1"]
pub type E1R = crate::FieldReader;
#[doc = "Field `E1` writer - 6:4\\]
DMA Queue Number for event #1"]
pub type E1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES17` reader - 7:7\\]
RESERVE FIELD"]
pub type Res17R = crate::BitReader;
#[doc = "Field `RES17` writer - 7:7\\]
RESERVE FIELD"]
pub type Res17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E2` reader - 10:8\\]
DMA Queue Number for event #2"]
pub type E2R = crate::FieldReader;
#[doc = "Field `E2` writer - 10:8\\]
DMA Queue Number for event #2"]
pub type E2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES16` reader - 11:11\\]
RESERVE FIELD"]
pub type Res16R = crate::BitReader;
#[doc = "Field `RES16` writer - 11:11\\]
RESERVE FIELD"]
pub type Res16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E3` reader - 14:12\\]
DMA Queue Number for event #3"]
pub type E3R = crate::FieldReader;
#[doc = "Field `E3` writer - 14:12\\]
DMA Queue Number for event #3"]
pub type E3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES15` reader - 15:15\\]
RESERVE FIELD"]
pub type Res15R = crate::BitReader;
#[doc = "Field `RES15` writer - 15:15\\]
RESERVE FIELD"]
pub type Res15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E4` reader - 18:16\\]
DMA Queue Number for event #4"]
pub type E4R = crate::FieldReader;
#[doc = "Field `E4` writer - 18:16\\]
DMA Queue Number for event #4"]
pub type E4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES14` reader - 19:19\\]
RESERVE FIELD"]
pub type Res14R = crate::BitReader;
#[doc = "Field `RES14` writer - 19:19\\]
RESERVE FIELD"]
pub type Res14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E5` reader - 22:20\\]
DMA Queue Number for event #5"]
pub type E5R = crate::FieldReader;
#[doc = "Field `E5` writer - 22:20\\]
DMA Queue Number for event #5"]
pub type E5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES13` reader - 23:23\\]
RESERVE FIELD"]
pub type Res13R = crate::BitReader;
#[doc = "Field `RES13` writer - 23:23\\]
RESERVE FIELD"]
pub type Res13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E6` reader - 26:24\\]
DMA Queue Number for event #6"]
pub type E6R = crate::FieldReader;
#[doc = "Field `E6` writer - 26:24\\]
DMA Queue Number for event #6"]
pub type E6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES12` reader - 27:27\\]
RESERVE FIELD"]
pub type Res12R = crate::BitReader;
#[doc = "Field `RES12` writer - 27:27\\]
RESERVE FIELD"]
pub type Res12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E7` reader - 30:28\\]
DMA Queue Number for event #7"]
pub type E7R = crate::FieldReader;
#[doc = "Field `E7` writer - 30:28\\]
DMA Queue Number for event #7"]
pub type E7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES11` reader - 31:31\\]
RESERVE FIELD"]
pub type Res11R = crate::BitReader;
#[doc = "Field `RES11` writer - 31:31\\]
RESERVE FIELD"]
pub type Res11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
DMA Queue Number for event #0"]
    #[inline(always)]
    pub fn e0(&self) -> E0R {
        E0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res18(&self) -> Res18R {
        Res18R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
DMA Queue Number for event #1"]
    #[inline(always)]
    pub fn e1(&self) -> E1R {
        E1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res17(&self) -> Res17R {
        Res17R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
DMA Queue Number for event #2"]
    #[inline(always)]
    pub fn e2(&self) -> E2R {
        E2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res16(&self) -> Res16R {
        Res16R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
DMA Queue Number for event #3"]
    #[inline(always)]
    pub fn e3(&self) -> E3R {
        E3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res15(&self) -> Res15R {
        Res15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
DMA Queue Number for event #4"]
    #[inline(always)]
    pub fn e4(&self) -> E4R {
        E4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res14(&self) -> Res14R {
        Res14R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - 22:20\\]
DMA Queue Number for event #5"]
    #[inline(always)]
    pub fn e5(&self) -> E5R {
        E5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res13(&self) -> Res13R {
        Res13R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
DMA Queue Number for event #6"]
    #[inline(always)]
    pub fn e6(&self) -> E6R {
        E6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res12(&self) -> Res12R {
        Res12R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
DMA Queue Number for event #7"]
    #[inline(always)]
    pub fn e7(&self) -> E7R {
        E7R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res11(&self) -> Res11R {
        Res11R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
DMA Queue Number for event #0"]
    #[inline(always)]
    #[must_use]
    pub fn e0(&mut self) -> E0W<DmaqnumnSpec> {
        E0W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res18(&mut self) -> Res18W<DmaqnumnSpec> {
        Res18W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
DMA Queue Number for event #1"]
    #[inline(always)]
    #[must_use]
    pub fn e1(&mut self) -> E1W<DmaqnumnSpec> {
        E1W::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res17(&mut self) -> Res17W<DmaqnumnSpec> {
        Res17W::new(self, 7)
    }
    #[doc = "Bits 8:10 - 10:8\\]
DMA Queue Number for event #2"]
    #[inline(always)]
    #[must_use]
    pub fn e2(&mut self) -> E2W<DmaqnumnSpec> {
        E2W::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res16(&mut self) -> Res16W<DmaqnumnSpec> {
        Res16W::new(self, 11)
    }
    #[doc = "Bits 12:14 - 14:12\\]
DMA Queue Number for event #3"]
    #[inline(always)]
    #[must_use]
    pub fn e3(&mut self) -> E3W<DmaqnumnSpec> {
        E3W::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res15(&mut self) -> Res15W<DmaqnumnSpec> {
        Res15W::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
DMA Queue Number for event #4"]
    #[inline(always)]
    #[must_use]
    pub fn e4(&mut self) -> E4W<DmaqnumnSpec> {
        E4W::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res14(&mut self) -> Res14W<DmaqnumnSpec> {
        Res14W::new(self, 19)
    }
    #[doc = "Bits 20:22 - 22:20\\]
DMA Queue Number for event #5"]
    #[inline(always)]
    #[must_use]
    pub fn e5(&mut self) -> E5W<DmaqnumnSpec> {
        E5W::new(self, 20)
    }
    #[doc = "Bit 23 - 23:23\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res13(&mut self) -> Res13W<DmaqnumnSpec> {
        Res13W::new(self, 23)
    }
    #[doc = "Bits 24:26 - 26:24\\]
DMA Queue Number for event #6"]
    #[inline(always)]
    #[must_use]
    pub fn e6(&mut self) -> E6W<DmaqnumnSpec> {
        E6W::new(self, 24)
    }
    #[doc = "Bit 27 - 27:27\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res12(&mut self) -> Res12W<DmaqnumnSpec> {
        Res12W::new(self, 27)
    }
    #[doc = "Bits 28:30 - 30:28\\]
DMA Queue Number for event #7"]
    #[inline(always)]
    #[must_use]
    pub fn e7(&mut self) -> E7W<DmaqnumnSpec> {
        E7W::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res11(&mut self) -> Res11W<DmaqnumnSpec> {
        Res11W::new(self, 31)
    }
}
#[doc = "DMA Queue Number Register n Contains the Event queue number to be used for the corresponding DMA Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaqnumn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaqnumn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaqnumnSpec;
impl crate::RegisterSpec for DmaqnumnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaqnumn::R`](R) reader structure"]
impl crate::Readable for DmaqnumnSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaqnumn::W`](W) writer structure"]
impl crate::Writable for DmaqnumnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAQNUMN to value 0"]
impl crate::Resettable for DmaqnumnSpec {
    const RESET_VALUE: u32 = 0;
}
