#[doc = "Register `IPCFGCLKGATE2` reader"]
pub type R = crate::R<Ipcfgclkgate2Spec>;
#[doc = "Register `IPCFGCLKGATE2` writer"]
pub type W = crate::W<Ipcfgclkgate2Spec>;
#[doc = "Field `gio` reader - 2:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type GioR = crate::FieldReader;
#[doc = "Field `gio` writer - 2:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type GioW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rs232` reader - 5:3\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type Rs232R = crate::FieldReader;
#[doc = "Field `rs232` writer - 5:3\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type Rs232W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hwass` reader - 8:6\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type HwassR = crate::FieldReader;
#[doc = "Field `hwass` writer - 8:6\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type HwassW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pcr5` reader - 11:9\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type Pcr5R = crate::FieldReader;
#[doc = "Field `pcr5` writer - 11:9\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type Pcr5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pcr6` reader - 14:12\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type Pcr6R = crate::FieldReader;
#[doc = "Field `pcr6` writer - 14:12\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
pub type Pcr6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn gio(&self) -> GioR {
        GioR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn rs232(&self) -> Rs232R {
        Rs232R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn hwass(&self) -> HwassR {
        HwassR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn pcr5(&self) -> Pcr5R {
        Pcr5R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    pub fn pcr6(&self) -> Pcr6R {
        Pcr6R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn gio(&mut self) -> GioW<Ipcfgclkgate2Spec> {
        GioW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn rs232(&mut self) -> Rs232W<Ipcfgclkgate2Spec> {
        Rs232W::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn hwass(&mut self) -> HwassW<Ipcfgclkgate2Spec> {
        HwassW::new(self, 6)
    }
    #[doc = "Bits 9:11 - 11:9\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn pcr5(&mut self) -> Pcr5W<Ipcfgclkgate2Spec> {
        Pcr5W::new(self, 9)
    }
    #[doc = "Bits 12:14 - 14:12\\]
0x0 : Enable the Clock 0x7 : Gate the clock"]
    #[inline(always)]
    #[must_use]
    pub fn pcr6(&mut self) -> Pcr6W<Ipcfgclkgate2Spec> {
        Pcr6W::new(self, 12)
    }
}
#[doc = "IPCFGCLKGATE2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipcfgclkgate2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcfgclkgate2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipcfgclkgate2Spec;
impl crate::RegisterSpec for Ipcfgclkgate2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipcfgclkgate2::R`](R) reader structure"]
impl crate::Readable for Ipcfgclkgate2Spec {}
#[doc = "`write(|w| ..)` method takes [`ipcfgclkgate2::W`](W) writer structure"]
impl crate::Writable for Ipcfgclkgate2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPCFGCLKGATE2 to value 0"]
impl crate::Resettable for Ipcfgclkgate2Spec {
    const RESET_VALUE: u32 = 0;
}
