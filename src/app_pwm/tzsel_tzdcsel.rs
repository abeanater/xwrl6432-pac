#[doc = "Register `TZSEL_TZDCSEL` reader"]
pub type R = crate::R<TzselTzdcselSpec>;
#[doc = "Register `TZSEL_TZDCSEL` writer"]
pub type W = crate::W<TzselTzdcselSpec>;
#[doc = "Field `TZSEL_CBC1` reader - 0:0\\]
Trip-zone 1 (TZ1) Select 0 Disable TZ1 as a CBC trip source for this ePWM module 1 Enable TZ1 as a CBC trip source for this ePWM module"]
pub type TzselCbc1R = crate::BitReader;
#[doc = "Field `TZSEL_CBC1` writer - 0:0\\]
Trip-zone 1 (TZ1) Select 0 Disable TZ1 as a CBC trip source for this ePWM module 1 Enable TZ1 as a CBC trip source for this ePWM module"]
pub type TzselCbc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_CBC2` reader - 1:1\\]
Trip-zone 2 (TZ2) Select 0 Disable TZ2 as a CBC trip source for this ePWM module 1 Enable TZ2 as a CBC trip source for this ePWM module"]
pub type TzselCbc2R = crate::BitReader;
#[doc = "Field `TZSEL_CBC2` writer - 1:1\\]
Trip-zone 2 (TZ2) Select 0 Disable TZ2 as a CBC trip source for this ePWM module 1 Enable TZ2 as a CBC trip source for this ePWM module"]
pub type TzselCbc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_CBC3` reader - 2:2\\]
Trip-zone 3 (TZ3) Select 0 Disable TZ3 as a CBC trip source for this ePWM module 1 Enable TZ3 as a CBC trip source for this ePWM module"]
pub type TzselCbc3R = crate::BitReader;
#[doc = "Field `TZSEL_CBC3` writer - 2:2\\]
Trip-zone 3 (TZ3) Select 0 Disable TZ3 as a CBC trip source for this ePWM module 1 Enable TZ3 as a CBC trip source for this ePWM module"]
pub type TzselCbc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_CBC4` reader - 3:3\\]
Trip-zone 4 (TZ4) Select 0 Disable TZ4 as a CBC trip source for this ePWM module 1 Enable TZ4 as a CBC trip source for this ePWM module"]
pub type TzselCbc4R = crate::BitReader;
#[doc = "Field `TZSEL_CBC4` writer - 3:3\\]
Trip-zone 4 (TZ4) Select 0 Disable TZ4 as a CBC trip source for this ePWM module 1 Enable TZ4 as a CBC trip source for this ePWM module"]
pub type TzselCbc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_CBC5` reader - 4:4\\]
Trip-zone 5 (TZ5) Select 0 Disable TZ5 as a CBC trip source for this ePWM module 1 Enable TZ5 as a CBC trip source for this ePWM module"]
pub type TzselCbc5R = crate::BitReader;
#[doc = "Field `TZSEL_CBC5` writer - 4:4\\]
Trip-zone 5 (TZ5) Select 0 Disable TZ5 as a CBC trip source for this ePWM module 1 Enable TZ5 as a CBC trip source for this ePWM module"]
pub type TzselCbc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_CBC6` reader - 5:5\\]
Trip-zone 6 (TZ6) Select 0 Disable TZ6 as a CBC trip source for this ePWM module 1 Enable TZ6 as a CBC trip source for this ePWM module"]
pub type TzselCbc6R = crate::BitReader;
#[doc = "Field `TZSEL_CBC6` writer - 5:5\\]
Trip-zone 6 (TZ6) Select 0 Disable TZ6 as a CBC trip source for this ePWM module 1 Enable TZ6 as a CBC trip source for this ePWM module"]
pub type TzselCbc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_DCAEVT2` reader - 6:6\\]
Digital Compare Output A Event 2 Select 0 Disable DCAEVT2 as a CBC trip source for this ePWM module 1 Enable DCAEVT2 as a CBC trip source for this ePWM module"]
pub type TzselDcaevt2R = crate::BitReader;
#[doc = "Field `TZSEL_DCAEVT2` writer - 6:6\\]
Digital Compare Output A Event 2 Select 0 Disable DCAEVT2 as a CBC trip source for this ePWM module 1 Enable DCAEVT2 as a CBC trip source for this ePWM module"]
pub type TzselDcaevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_DCBEVT2` reader - 7:7\\]
Digital Compare Output B Event 2 Select 0 Disable DCBEVT2 as a CBC trip source for this ePWM module 1 Enable DCBEVT2 as a CBC trip source for this ePWM module"]
pub type TzselDcbevt2R = crate::BitReader;
#[doc = "Field `TZSEL_DCBEVT2` writer - 7:7\\]
Digital Compare Output B Event 2 Select 0 Disable DCBEVT2 as a CBC trip source for this ePWM module 1 Enable DCBEVT2 as a CBC trip source for this ePWM module"]
pub type TzselDcbevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_OSHT1` reader - 8:8\\]
Trip-zone 1 (TZ1) Select 0 Disable TZ1 as a one-shot trip source for this ePWM module 1 Enable TZ1 as a one-shot trip source for this ePWM module"]
pub type TzselOsht1R = crate::BitReader;
#[doc = "Field `TZSEL_OSHT1` writer - 8:8\\]
Trip-zone 1 (TZ1) Select 0 Disable TZ1 as a one-shot trip source for this ePWM module 1 Enable TZ1 as a one-shot trip source for this ePWM module"]
pub type TzselOsht1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_OSHT2` reader - 9:9\\]
Trip-zone 2 (TZ2) Select 0 Disable TZ2 as a one-shot trip source for this ePWM module 1 Enable TZ2 as a one-shot trip source for this ePWM module"]
pub type TzselOsht2R = crate::BitReader;
#[doc = "Field `TZSEL_OSHT2` writer - 9:9\\]
Trip-zone 2 (TZ2) Select 0 Disable TZ2 as a one-shot trip source for this ePWM module 1 Enable TZ2 as a one-shot trip source for this ePWM module"]
pub type TzselOsht2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_OSHT3` reader - 10:10\\]
Trip-zone 3 (TZ3) Select 0 Disable TZ3 as a one-shot trip source for this ePWM module 1 Enable TZ3 as a one-shot trip source for this ePWM module"]
pub type TzselOsht3R = crate::BitReader;
#[doc = "Field `TZSEL_OSHT3` writer - 10:10\\]
Trip-zone 3 (TZ3) Select 0 Disable TZ3 as a one-shot trip source for this ePWM module 1 Enable TZ3 as a one-shot trip source for this ePWM module"]
pub type TzselOsht3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_OSHT4` reader - 11:11\\]
Trip-zone 4 (TZ4) Select 0 Disable TZ4 as a one-shot trip source for this ePWM module 1 Enable TZ4 as a one-shot trip source for this ePWM module"]
pub type TzselOsht4R = crate::BitReader;
#[doc = "Field `TZSEL_OSHT4` writer - 11:11\\]
Trip-zone 4 (TZ4) Select 0 Disable TZ4 as a one-shot trip source for this ePWM module 1 Enable TZ4 as a one-shot trip source for this ePWM module"]
pub type TzselOsht4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_OSHT5` reader - 12:12\\]
Trip-zone 5 (TZ5) Select 0 Disable TZ5 as a one-shot trip source for this ePWM module 1 Enable TZ5 as a one-shot trip source for this ePWM module"]
pub type TzselOsht5R = crate::BitReader;
#[doc = "Field `TZSEL_OSHT5` writer - 12:12\\]
Trip-zone 5 (TZ5) Select 0 Disable TZ5 as a one-shot trip source for this ePWM module 1 Enable TZ5 as a one-shot trip source for this ePWM module"]
pub type TzselOsht5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_OSHT6` reader - 13:13\\]
Trip-zone 6 (TZ6) Select 0 Disable TZ6 as a one-shot trip source for this ePWM module. 1 Enable TZ6 as a one-shot trip source for this ePWM module"]
pub type TzselOsht6R = crate::BitReader;
#[doc = "Field `TZSEL_OSHT6` writer - 13:13\\]
Trip-zone 6 (TZ6) Select 0 Disable TZ6 as a one-shot trip source for this ePWM module. 1 Enable TZ6 as a one-shot trip source for this ePWM module"]
pub type TzselOsht6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_DCAEVT1` reader - 14:14\\]
Digital Compare Output A Event 1 Select 0 Disable DCAEVT1 as one-shot-trip source for this ePWM module. 1 Enable DCAEVT1 as one-shot-trip source for this ePWM module"]
pub type TzselDcaevt1R = crate::BitReader;
#[doc = "Field `TZSEL_DCAEVT1` writer - 14:14\\]
Digital Compare Output A Event 1 Select 0 Disable DCAEVT1 as one-shot-trip source for this ePWM module. 1 Enable DCAEVT1 as one-shot-trip source for this ePWM module"]
pub type TzselDcaevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZSEL_DCBEVT1` reader - 15:15\\]
Digital Compare Output B Event 1 Select 0 Disable DCBEVT1 as one-shot-trip source for this ePWM module. 1 Enable DCBEVT1 as one-shot-trip source for this ePWM module"]
pub type TzselDcbevt1R = crate::BitReader;
#[doc = "Field `TZSEL_DCBEVT1` writer - 15:15\\]
Digital Compare Output B Event 1 Select 0 Disable DCBEVT1 as one-shot-trip source for this ePWM module. 1 Enable DCBEVT1 as one-shot-trip source for this ePWM module"]
pub type TzselDcbevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZDCSEL_DCAEVT1` reader - 18:16\\]
Digital Compare Output A Event 1 Selection 0 Event disabled 1h DCAH = low, DCAL = don't care 2h DCAH = high, DCAL = don't care 3h DCAL = low, DCAH = don't care 4h DCAL = high, DCAH = don't care 5h DCAL = high, DCAH = low 6h-7h Reserved"]
pub type TzdcselDcaevt1R = crate::FieldReader;
#[doc = "Field `TZDCSEL_DCAEVT1` writer - 18:16\\]
Digital Compare Output A Event 1 Selection 0 Event disabled 1h DCAH = low, DCAL = don't care 2h DCAH = high, DCAL = don't care 3h DCAL = low, DCAH = don't care 4h DCAL = high, DCAH = don't care 5h DCAL = high, DCAH = low 6h-7h Reserved"]
pub type TzdcselDcaevt1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TZDCSEL_DCAEVT2` reader - 21:19\\]
Digital Compare Output A Event 2 Selection 0 Event disabled 1h DCAH = low, DCAL = don't care 2h DCAH = high, DCAL = don't care 3h DCAL = low, DCAH = don't care 4h DCAL = high, DCAH = don't care 5h DCAL = high, DCAH = low 6h-7h Reserved"]
pub type TzdcselDcaevt2R = crate::FieldReader;
#[doc = "Field `TZDCSEL_DCAEVT2` writer - 21:19\\]
Digital Compare Output A Event 2 Selection 0 Event disabled 1h DCAH = low, DCAL = don't care 2h DCAH = high, DCAL = don't care 3h DCAL = low, DCAH = don't care 4h DCAL = high, DCAH = don't care 5h DCAL = high, DCAH = low 6h-7h Reserved"]
pub type TzdcselDcaevt2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TZDCSEL_DCBEVT1` reader - 24:22\\]
Digital Compare Output B Event 1 Selection 0 Event disabled 1h DCBH = low, DCBL = don't care 2h DCBH = high, DCBL = don't care 3h DCBL = low, DCBH = don't care 4h DCBL = high, DCBH = don't care 5h DCBL = high, DCBH = low 6h-7h Reserved"]
pub type TzdcselDcbevt1R = crate::FieldReader;
#[doc = "Field `TZDCSEL_DCBEVT1` writer - 24:22\\]
Digital Compare Output B Event 1 Selection 0 Event disabled 1h DCBH = low, DCBL = don't care 2h DCBH = high, DCBL = don't care 3h DCBL = low, DCBH = don't care 4h DCBL = high, DCBH = don't care 5h DCBL = high, DCBH = low 6h-7h Reserved"]
pub type TzdcselDcbevt1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TZDCSEL_DCBEVT2` reader - 27:25\\]
Digital Compare Output B Event 2 Selection 0 Event disabled 1h DCBH = low, DCBL = don't care 2h DCBH = high, DCBL = don't care 3h DCBL = low, DCBH = don't care 4h DCBL = high, DCBH = don't care 5h DCBL = high, DCBH = low 6h-7h Reserved"]
pub type TzdcselDcbevt2R = crate::FieldReader;
#[doc = "Field `TZDCSEL_DCBEVT2` writer - 27:25\\]
Digital Compare Output B Event 2 Selection 0 Event disabled 1h DCBH = low, DCBL = don't care 2h DCBH = high, DCBL = don't care 3h DCBL = low, DCBH = don't care 4h DCBL = high, DCBH = don't care 5h DCBL = high, DCBH = low 6h-7h Reserved"]
pub type TzdcselDcbevt2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - 31:28\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 31:28\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Trip-zone 1 (TZ1) Select 0 Disable TZ1 as a CBC trip source for this ePWM module 1 Enable TZ1 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_cbc1(&self) -> TzselCbc1R {
        TzselCbc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Trip-zone 2 (TZ2) Select 0 Disable TZ2 as a CBC trip source for this ePWM module 1 Enable TZ2 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_cbc2(&self) -> TzselCbc2R {
        TzselCbc2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Trip-zone 3 (TZ3) Select 0 Disable TZ3 as a CBC trip source for this ePWM module 1 Enable TZ3 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_cbc3(&self) -> TzselCbc3R {
        TzselCbc3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Trip-zone 4 (TZ4) Select 0 Disable TZ4 as a CBC trip source for this ePWM module 1 Enable TZ4 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_cbc4(&self) -> TzselCbc4R {
        TzselCbc4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Trip-zone 5 (TZ5) Select 0 Disable TZ5 as a CBC trip source for this ePWM module 1 Enable TZ5 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_cbc5(&self) -> TzselCbc5R {
        TzselCbc5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Trip-zone 6 (TZ6) Select 0 Disable TZ6 as a CBC trip source for this ePWM module 1 Enable TZ6 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_cbc6(&self) -> TzselCbc6R {
        TzselCbc6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Digital Compare Output A Event 2 Select 0 Disable DCAEVT2 as a CBC trip source for this ePWM module 1 Enable DCAEVT2 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_dcaevt2(&self) -> TzselDcaevt2R {
        TzselDcaevt2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Digital Compare Output B Event 2 Select 0 Disable DCBEVT2 as a CBC trip source for this ePWM module 1 Enable DCBEVT2 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_dcbevt2(&self) -> TzselDcbevt2R {
        TzselDcbevt2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Trip-zone 1 (TZ1) Select 0 Disable TZ1 as a one-shot trip source for this ePWM module 1 Enable TZ1 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_osht1(&self) -> TzselOsht1R {
        TzselOsht1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Trip-zone 2 (TZ2) Select 0 Disable TZ2 as a one-shot trip source for this ePWM module 1 Enable TZ2 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_osht2(&self) -> TzselOsht2R {
        TzselOsht2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Trip-zone 3 (TZ3) Select 0 Disable TZ3 as a one-shot trip source for this ePWM module 1 Enable TZ3 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_osht3(&self) -> TzselOsht3R {
        TzselOsht3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Trip-zone 4 (TZ4) Select 0 Disable TZ4 as a one-shot trip source for this ePWM module 1 Enable TZ4 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_osht4(&self) -> TzselOsht4R {
        TzselOsht4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Trip-zone 5 (TZ5) Select 0 Disable TZ5 as a one-shot trip source for this ePWM module 1 Enable TZ5 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_osht5(&self) -> TzselOsht5R {
        TzselOsht5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Trip-zone 6 (TZ6) Select 0 Disable TZ6 as a one-shot trip source for this ePWM module. 1 Enable TZ6 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_osht6(&self) -> TzselOsht6R {
        TzselOsht6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Digital Compare Output A Event 1 Select 0 Disable DCAEVT1 as one-shot-trip source for this ePWM module. 1 Enable DCAEVT1 as one-shot-trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_dcaevt1(&self) -> TzselDcaevt1R {
        TzselDcaevt1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Digital Compare Output B Event 1 Select 0 Disable DCBEVT1 as one-shot-trip source for this ePWM module. 1 Enable DCBEVT1 as one-shot-trip source for this ePWM module"]
    #[inline(always)]
    pub fn tzsel_dcbevt1(&self) -> TzselDcbevt1R {
        TzselDcbevt1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Digital Compare Output A Event 1 Selection 0 Event disabled 1h DCAH = low, DCAL = don't care 2h DCAH = high, DCAL = don't care 3h DCAL = low, DCAH = don't care 4h DCAL = high, DCAH = don't care 5h DCAL = high, DCAH = low 6h-7h Reserved"]
    #[inline(always)]
    pub fn tzdcsel_dcaevt1(&self) -> TzdcselDcaevt1R {
        TzdcselDcaevt1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Digital Compare Output A Event 2 Selection 0 Event disabled 1h DCAH = low, DCAL = don't care 2h DCAH = high, DCAL = don't care 3h DCAL = low, DCAH = don't care 4h DCAL = high, DCAH = don't care 5h DCAL = high, DCAH = low 6h-7h Reserved"]
    #[inline(always)]
    pub fn tzdcsel_dcaevt2(&self) -> TzdcselDcaevt2R {
        TzdcselDcaevt2R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - 24:22\\]
Digital Compare Output B Event 1 Selection 0 Event disabled 1h DCBH = low, DCBL = don't care 2h DCBH = high, DCBL = don't care 3h DCBL = low, DCBH = don't care 4h DCBL = high, DCBH = don't care 5h DCBL = high, DCBH = low 6h-7h Reserved"]
    #[inline(always)]
    pub fn tzdcsel_dcbevt1(&self) -> TzdcselDcbevt1R {
        TzdcselDcbevt1R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - 27:25\\]
Digital Compare Output B Event 2 Selection 0 Event disabled 1h DCBH = low, DCBL = don't care 2h DCBH = high, DCBL = don't care 3h DCBL = low, DCBH = don't care 4h DCBL = high, DCBH = don't care 5h DCBL = high, DCBH = low 6h-7h Reserved"]
    #[inline(always)]
    pub fn tzdcsel_dcbevt2(&self) -> TzdcselDcbevt2R {
        TzdcselDcbevt2R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Trip-zone 1 (TZ1) Select 0 Disable TZ1 as a CBC trip source for this ePWM module 1 Enable TZ1 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_cbc1(&mut self) -> TzselCbc1W<TzselTzdcselSpec> {
        TzselCbc1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Trip-zone 2 (TZ2) Select 0 Disable TZ2 as a CBC trip source for this ePWM module 1 Enable TZ2 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_cbc2(&mut self) -> TzselCbc2W<TzselTzdcselSpec> {
        TzselCbc2W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Trip-zone 3 (TZ3) Select 0 Disable TZ3 as a CBC trip source for this ePWM module 1 Enable TZ3 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_cbc3(&mut self) -> TzselCbc3W<TzselTzdcselSpec> {
        TzselCbc3W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Trip-zone 4 (TZ4) Select 0 Disable TZ4 as a CBC trip source for this ePWM module 1 Enable TZ4 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_cbc4(&mut self) -> TzselCbc4W<TzselTzdcselSpec> {
        TzselCbc4W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Trip-zone 5 (TZ5) Select 0 Disable TZ5 as a CBC trip source for this ePWM module 1 Enable TZ5 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_cbc5(&mut self) -> TzselCbc5W<TzselTzdcselSpec> {
        TzselCbc5W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Trip-zone 6 (TZ6) Select 0 Disable TZ6 as a CBC trip source for this ePWM module 1 Enable TZ6 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_cbc6(&mut self) -> TzselCbc6W<TzselTzdcselSpec> {
        TzselCbc6W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Digital Compare Output A Event 2 Select 0 Disable DCAEVT2 as a CBC trip source for this ePWM module 1 Enable DCAEVT2 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_dcaevt2(&mut self) -> TzselDcaevt2W<TzselTzdcselSpec> {
        TzselDcaevt2W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Digital Compare Output B Event 2 Select 0 Disable DCBEVT2 as a CBC trip source for this ePWM module 1 Enable DCBEVT2 as a CBC trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_dcbevt2(&mut self) -> TzselDcbevt2W<TzselTzdcselSpec> {
        TzselDcbevt2W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Trip-zone 1 (TZ1) Select 0 Disable TZ1 as a one-shot trip source for this ePWM module 1 Enable TZ1 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_osht1(&mut self) -> TzselOsht1W<TzselTzdcselSpec> {
        TzselOsht1W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Trip-zone 2 (TZ2) Select 0 Disable TZ2 as a one-shot trip source for this ePWM module 1 Enable TZ2 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_osht2(&mut self) -> TzselOsht2W<TzselTzdcselSpec> {
        TzselOsht2W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Trip-zone 3 (TZ3) Select 0 Disable TZ3 as a one-shot trip source for this ePWM module 1 Enable TZ3 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_osht3(&mut self) -> TzselOsht3W<TzselTzdcselSpec> {
        TzselOsht3W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Trip-zone 4 (TZ4) Select 0 Disable TZ4 as a one-shot trip source for this ePWM module 1 Enable TZ4 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_osht4(&mut self) -> TzselOsht4W<TzselTzdcselSpec> {
        TzselOsht4W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Trip-zone 5 (TZ5) Select 0 Disable TZ5 as a one-shot trip source for this ePWM module 1 Enable TZ5 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_osht5(&mut self) -> TzselOsht5W<TzselTzdcselSpec> {
        TzselOsht5W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Trip-zone 6 (TZ6) Select 0 Disable TZ6 as a one-shot trip source for this ePWM module. 1 Enable TZ6 as a one-shot trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_osht6(&mut self) -> TzselOsht6W<TzselTzdcselSpec> {
        TzselOsht6W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Digital Compare Output A Event 1 Select 0 Disable DCAEVT1 as one-shot-trip source for this ePWM module. 1 Enable DCAEVT1 as one-shot-trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_dcaevt1(&mut self) -> TzselDcaevt1W<TzselTzdcselSpec> {
        TzselDcaevt1W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Digital Compare Output B Event 1 Select 0 Disable DCBEVT1 as one-shot-trip source for this ePWM module. 1 Enable DCBEVT1 as one-shot-trip source for this ePWM module"]
    #[inline(always)]
    #[must_use]
    pub fn tzsel_dcbevt1(&mut self) -> TzselDcbevt1W<TzselTzdcselSpec> {
        TzselDcbevt1W::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Digital Compare Output A Event 1 Selection 0 Event disabled 1h DCAH = low, DCAL = don't care 2h DCAH = high, DCAL = don't care 3h DCAL = low, DCAH = don't care 4h DCAL = high, DCAH = don't care 5h DCAL = high, DCAH = low 6h-7h Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tzdcsel_dcaevt1(&mut self) -> TzdcselDcaevt1W<TzselTzdcselSpec> {
        TzdcselDcaevt1W::new(self, 16)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Digital Compare Output A Event 2 Selection 0 Event disabled 1h DCAH = low, DCAL = don't care 2h DCAH = high, DCAL = don't care 3h DCAL = low, DCAH = don't care 4h DCAL = high, DCAH = don't care 5h DCAL = high, DCAH = low 6h-7h Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tzdcsel_dcaevt2(&mut self) -> TzdcselDcaevt2W<TzselTzdcselSpec> {
        TzdcselDcaevt2W::new(self, 19)
    }
    #[doc = "Bits 22:24 - 24:22\\]
Digital Compare Output B Event 1 Selection 0 Event disabled 1h DCBH = low, DCBL = don't care 2h DCBH = high, DCBL = don't care 3h DCBL = low, DCBH = don't care 4h DCBL = high, DCBH = don't care 5h DCBL = high, DCBH = low 6h-7h Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tzdcsel_dcbevt1(&mut self) -> TzdcselDcbevt1W<TzselTzdcselSpec> {
        TzdcselDcbevt1W::new(self, 22)
    }
    #[doc = "Bits 25:27 - 27:25\\]
Digital Compare Output B Event 2 Selection 0 Event disabled 1h DCBH = low, DCBL = don't care 2h DCBH = high, DCBL = don't care 3h DCBL = low, DCBH = don't care 4h DCBL = high, DCBH = don't care 5h DCBL = high, DCBH = low 6h-7h Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tzdcsel_dcbevt2(&mut self) -> TzdcselDcbevt2W<TzselTzdcselSpec> {
        TzdcselDcbevt2W::new(self, 25)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<TzselTzdcselSpec> {
        Reserved1W::new(self, 28)
    }
}
#[doc = "Trip Zone Digital Compare Select Register/ Trip-Zone Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzsel_tzdcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzsel_tzdcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzselTzdcselSpec;
impl crate::RegisterSpec for TzselTzdcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzsel_tzdcsel::R`](R) reader structure"]
impl crate::Readable for TzselTzdcselSpec {}
#[doc = "`write(|w| ..)` method takes [`tzsel_tzdcsel::W`](W) writer structure"]
impl crate::Writable for TzselTzdcselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZSEL_TZDCSEL to value 0"]
impl crate::Resettable for TzselTzdcselSpec {
    const RESET_VALUE: u32 = 0;
}
